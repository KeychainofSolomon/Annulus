use std::collections::BTreeSet;

use super::rite::{ReqForm, Rite};
use super::sha256::hash_hex;
use super::state::{
    digest_faculty, digest_manifest, fingerprint_effects, fingerprint_validator, verify, State,
};
use super::types::{
    Addressee, Cap, CircleId, CommitResult, Content, Contrib, Ending, ExamAns, Inst, InvId,
    InvStatus, IxEntry, KeyEntry, Label, Made, Mark, Outcome, PStatus, Phase, Pid, Proposal, ReqId,
    Request, Seal, Sealed, Value, ValueId, Verdict, WorkOutcome, Worker, WorkerId, DENIAL,
    OPERATOR,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    NotEnabled(&'static str),
    Denied(&'static str),
    LaunchRefused(&'static str),
}

pub type Step<T> = Result<T, Refusal>;

impl State {
    pub fn launch_checks(&self) -> Result<(), &'static str> {
        let auth = &self.auth;
        if !verify(
            &auth.owner,
            &digest_manifest(&self.constitution.manifest),
            &auth.manifest_signature,
        ) {
            return Err("the Constitution's manifest is not signed by the owner's wand");
        }
        if auth.revoked.contains(&auth.owner) {
            return Err("the owner's wand is revoked");
        }
        for (id, validator) in &self.constitution.validators {
            if self.constitution.manifest.get(id) != Some(&fingerprint_validator(&validator.name)) {
                return Err("a validator does not match the manifest");
            }
        }
        if !self.stores.index.intact() || self.stores.index.root() != auth.roots.index {
            return Err("the Index has been tampered with");
        }
        if !self.stores.vessel.intact() || self.stores.vessel.root() != auth.roots.vessel {
            return Err("the Vessel has been tampered with");
        }
        if !self.stores.keys.intact() || self.stores.keys.root() != auth.roots.keys {
            return Err("the store of sealed petitions has been tampered with");
        }
        if auth.wand.master_unlocked {
            return Err("the wand is unlocked as master");
        }
        if auth.wand.present {
            if auth.revoked.contains(&auth.wand.fingerprint) {
                return Err("the wand is revoked");
            }
            if !verify(
                &auth.wand.fingerprint,
                &digest_faculty(&self.faculty, &self.reserved),
                &auth.faculty_signature,
            ) {
                return Err("the faculty is not signed by the wand");
            }
        }
        Ok(())
    }

    pub fn launch(&mut self) -> Step<()> {
        if self.phase != Phase::PreLaunch {
            return Err(Refusal::NotEnabled("already launched"));
        }
        self.launch_checks().map_err(Refusal::LaunchRefused)?;
        if !self.auth.wand.present {
            self.faculty.clear();
            self.reserved.clear();
            self.ink_left.clear();
        }
        let granted: BTreeSet<String> = self.faculty.iter().map(|c| c.place.clone()).collect();
        let fingerprints = granted
            .iter()
            .filter_map(|p| {
                self.world
                    .get(p)
                    .map(|(d, m)| (p.clone(), hash_hex(d.as_bytes()), *m))
            })
            .collect();
        self.phase = Phase::Running;
        self.record(Label::Launch(fingerprints));
        Ok(())
    }

    fn operator(&self) -> Step<()> {
        if !self.running() {
            return Err(Refusal::NotEnabled("the run is not running"));
        }
        if self.current != Some(OPERATOR) {
            return Err(Refusal::NotEnabled("the Operator is not scheduled"));
        }
        Ok(())
    }

    fn deny<T>(&mut self, why: &'static str) -> Step<T> {
        let at = self.audit.len();
        let dismissed = self.open_invocations();
        self.record(Label::Fail(DENIAL, dismissed));
        self.end_run(Ending::Failure(DENIAL, at));
        Err(Refusal::Denied(why))
    }

    fn done(&mut self) {
        self.current = None;
    }

    pub fn eligible(&self, worker: WorkerId) -> bool {
        if worker == OPERATOR {
            return true;
        }
        self.workers.get(&worker).is_some_and(|w| {
            w.alive
                && w.queue.front().is_some_and(|p| {
                    self.props
                        .get(p)
                        .is_some_and(|p| p.status == PStatus::Pending)
                })
        })
    }

    pub fn schedule(&mut self, worker: WorkerId) -> Step<()> {
        if !self.running() || self.current.is_some() {
            return Err(Refusal::NotEnabled("a worker is already scheduled"));
        }
        if !self.eligible(worker) {
            return Err(Refusal::NotEnabled("the worker is not eligible"));
        }
        self.current = Some(worker);
        self.order.push(worker);
        self.entropy += 1;
        self.record(Label::Schedule(worker));
        Ok(())
    }

    pub fn open_circle(&mut self) -> Step<CircleId> {
        self.operator()?;
        let circle = self.fresh();
        self.circles.push(circle);
        self.record(Label::OpenC(circle));
        self.done();
        Ok(circle)
    }

    pub fn close_circle(&mut self) -> Step<()> {
        self.operator()?;
        let Some(&circle) = self.circles.last() else {
            return self.deny("no Circle is open");
        };
        self.terminate(|_, inst| inst.circle == circle, InvStatus::Released);
        self.circles.pop();
        self.record(Label::CloseC(circle));
        self.done();
        Ok(())
    }

    pub fn invoke(&mut self, rite: &dyn Rite, seal: &Seal) -> Step<InvId> {
        self.operator()?;
        let Some(&circle) = self.circles.last() else {
            return self.deny("INVOCA outside a Circle");
        };
        let admitted = rite.admit(self, seal, circle);
        let inv = self.fresh();
        let worker = self.fresh();
        let sealed = self.inscribed.contains(&(seal.clone(), circle));
        if !admitted || self.barred(seal) || !sealed {
            let dismissed = self.open_invocations();
            self.record(Label::Invoke(
                inv,
                seal.clone(),
                circle,
                worker,
                admitted,
                dismissed,
            ));
            let at = self.audit.len();
            self.end_run(Ending::Failure(DENIAL, at));
            return Err(Refusal::Denied("the invocation is refused"));
        }
        self.invs.insert(
            inv,
            Inst {
                seal: seal.clone(),
                circle,
                status: InvStatus::Live,
                main: worker,
                helpers: Vec::new(),
            },
        );
        self.workers.insert(
            worker,
            Worker {
                owner: Some(Addressee::Spirit(inv)),
                parent: Some(inv),
                handed: BTreeSet::new(),
                queue: Default::default(),
                alive: true,
                helper: false,
            },
        );
        self.record(Label::Invoke(
            inv,
            seal.clone(),
            circle,
            worker,
            true,
            Vec::new(),
        ));
        self.done();
        Ok(inv)
    }

    pub fn inscribe(&mut self, rite: &dyn Rite, seal: &Seal, class: &str) -> Step<()> {
        self.operator()?;
        let Some(&circle) = self.circles.last() else {
            return self.deny("INSCRIBE outside a Circle");
        };
        if !rite.ink_fits(self, seal, class) {
            return self.deny("this ink is not the seal's class");
        }
        match self.ink_left.get_mut(class) {
            Some(left) if *left > 0 => *left -= 1,
            _ => return self.deny("no ink of this class is left in the faculty"),
        }
        self.inscribed.insert((seal.clone(), circle));
        self.record(Label::Inscribe(seal.clone(), class.to_string(), circle));
        self.done();
        Ok(())
    }

    pub fn request(&mut self, rite: &dyn Rite, to: Addressee, form: &ReqForm) -> Step<Pid> {
        self.operator()?;
        let asked = rite.signature(self, form);
        if asked.iter().any(|c| !self.contained(&to, c)) {
            return self.deny("the request asks for more than the faculty grants");
        }
        let (author, worker) = match &to {
            Addressee::Spirit(inv) => match self.invs.get(inv) {
                Some(inst)
                    if inst.status == InvStatus::Live && self.circles.contains(&inst.circle) =>
                {
                    (inst.seal.clone(), inst.main)
                }
                _ => return self.deny("PETE to an invocation that is not open in an open Circle"),
            },
            Addressee::Angel(seal) => {
                if !self.angelic(seal) {
                    return self.deny("petition to a seal that is not an angel's");
                }
                (seal.clone(), self.fresh())
            }
        };
        let req: ReqId = self.fresh();
        let prop: Pid = self.fresh();
        self.reqs.insert(
            req,
            Request {
                to: to.clone(),
                handed: asked.clone(),
                prop,
                worker,
            },
        );
        self.props.insert(
            prop,
            Proposal {
                author: author.clone(),
                req,
                content: Content::Empty,
                status: PStatus::Pending,
                carries: vec![author.clone()],
            },
        );
        match &to {
            Addressee::Spirit(_) => {
                if let Some(w) = self.workers.get_mut(&worker) {
                    w.queue.push_back(prop);
                }
            }
            Addressee::Angel(_) => {
                self.workers.insert(
                    worker,
                    Worker {
                        owner: Some(to.clone()),
                        parent: None,
                        handed: asked.clone(),
                        queue: [prop].into_iter().collect(),
                        alive: true,
                        helper: false,
                    },
                );
            }
        }
        self.record(Label::Request(
            req,
            prop,
            to,
            worker,
            asked.into_iter().collect(),
        ));
        self.done();
        Ok(prop)
    }

    fn contrib_ok(
        &self,
        rite: &dyn Rite,
        worker: &Worker,
        req: &Request,
        prop: &Proposal,
        contrib: &Contrib,
    ) -> bool {
        match contrib {
            Contrib::Respond(v, _) => {
                prop.content == Content::Empty && !self.values.contains_key(v)
            }
            Contrib::Effects(es) => {
                matches!(prop.content, Content::Empty | Content::Work(_))
                    && (!worker.helper || es.iter().all(|e| worker.handed.contains(&e.needs())))
            }
            Contrib::Spawn(w, handed) => {
                let Some(inv) = worker.parent else {
                    return false;
                };
                let Some(inst) = self.invs.get(&inv) else {
                    return false;
                };
                !self.workers.contains_key(w)
                    && inst.helpers.len() < rite.legions(self, inv)
                    && handed.iter().all(|c| {
                        req.handed.contains(c) && (!worker.helper || worker.handed.contains(c))
                    })
            }
            Contrib::Idle => true,
        }
    }

    pub fn turn(
        &mut self,
        rite: &dyn Rite,
        worker: WorkerId,
        outcome: Outcome,
        contrib: Contrib,
        finish: bool,
    ) -> Step<()> {
        if !self.running() || self.current != Some(worker) || worker == OPERATOR {
            return Err(Refusal::NotEnabled("the worker is not scheduled"));
        }
        let Some(w) = self.workers.get(&worker).cloned() else {
            return Err(Refusal::NotEnabled("no such worker"));
        };
        let Some(&prop) = w.queue.front() else {
            return Err(Refusal::NotEnabled("the worker has no request"));
        };
        let Some(p) = self.props.get(&prop).cloned() else {
            return Err(Refusal::NotEnabled("no such proposal"));
        };
        let Some(req) = self.reqs.get(&p.req).cloned() else {
            return Err(Refusal::NotEnabled("no such request"));
        };
        if !w.alive || p.status != PStatus::Pending {
            return Err(Refusal::NotEnabled("nothing to work on"));
        }
        if outcome == Outcome::Occisio || !rite.will_open(self, worker, outcome) {
            return Err(Refusal::NotEnabled("that outcome of the will is not open"));
        }
        if finish && w.helper {
            return Err(Refusal::NotEnabled("a helper cannot finish the request"));
        }
        if !self.contrib_ok(rite, &w, &req, &p, &contrib) {
            return Err(Refusal::NotEnabled(
                "the contribution is not of the office's form",
            ));
        }
        let index = self.record(Label::Turn(worker, outcome, contrib.clone(), finish));
        let entry = self.props.get_mut(&prop).expect("checked above");
        match &contrib {
            Contrib::Respond(v, d) => {
                entry.content = Content::Response(*v, d.clone());
            }
            Contrib::Effects(es) => {
                let mut all = entry.content.effects();
                all.extend(es.iter().cloned());
                entry.content = Content::Work(all);
            }
            _ => {}
        }
        if finish {
            entry.status = PStatus::Complete;
        }
        if let Contrib::Respond(v, _) = &contrib {
            self.values.insert(
                *v,
                Value {
                    data: None,
                    mark: Mark::Quarantined,
                    origin: Some(index),
                    circle: None,
                },
            );
        }
        if let Contrib::Spawn(helper, handed) = &contrib {
            self.workers.insert(
                *helper,
                Worker {
                    owner: w.owner.clone(),
                    parent: w.parent,
                    handed: handed.clone(),
                    queue: [prop].into_iter().collect(),
                    alive: true,
                    helper: true,
                },
            );
            if let Some(inst) = w.parent.and_then(|i| self.invs.get_mut(&i)) {
                inst.helpers.push(*helper);
            }
        }
        if finish {
            if let Some(entry) = self.workers.get_mut(&worker) {
                entry.queue.pop_front();
            }
        }
        self.entropy += 1;
        self.done();
        Ok(())
    }

    pub fn kill(&mut self, rite: &dyn Rite, worker: WorkerId, outcome: Outcome) -> Step<()> {
        if !self.running() || self.current != Some(worker) {
            return Err(Refusal::NotEnabled("the worker is not scheduled"));
        }
        let able = self
            .workers
            .get(&worker)
            .is_some_and(|w| w.alive && w.parent.is_some());
        if !able || outcome != Outcome::Occisio || !rite.will_open(self, worker, outcome) {
            return Err(Refusal::NotEnabled("killing is not open"));
        }
        let dismissed = self.open_invocations();
        self.record(Label::Kill(worker, outcome, dismissed));
        self.end_run(Ending::Killed);
        Ok(())
    }

    pub fn examine(&mut self, rite: &dyn Rite, prop: Pid) -> Step<WorkOutcome> {
        self.operator()?;
        let Some(p) = self.props.get(&prop).cloned() else {
            return self.deny("EXAMINA of an unknown proposal");
        };
        match p.status {
            PStatus::Pending => return Err(Refusal::NotEnabled("the work is not complete")),
            PStatus::Complete | PStatus::Judged(WorkOutcome::Suspensum) => {}
            _ => return self.deny("this proposal cannot be examined again"),
        }
        let suspect = self.newly_suspect(&p.author, prop);
        if suspect {
            self.stores.index.push(IxEntry::Suspect(p.author.clone()));
        }
        let examiner = rite.examiner(self, prop);
        let (answer, verdict) = match &examiner {
            Some(e) if *e != p.author && !p.carries.contains(e) => {
                let v0 = rite.verdict(self, prop, e);
                let cb = rite.counterbalance(self, prop);
                let v = if suspect || self.doubtful(&p.author) {
                    Verdict::Hold
                } else {
                    v0.meet(cb)
                };
                (ExamAns::Examined(e.clone(), v0, cb), v)
            }
            _ => (ExamAns::NoExaminer(examiner.clone()), Verdict::Hold),
        };
        self.record(Label::Examine(prop, answer, verdict));
        let outcome = verdict.outcome();
        self.record(Label::Judge(prop, outcome));
        let final_outcome = match outcome {
            WorkOutcome::Ratum => self.commit(prop),
            WorkOutcome::Irritum => {
                self.void(prop);
                WorkOutcome::Irritum
            }
            WorkOutcome::Suspensum => {
                self.set_status(prop, PStatus::Judged(WorkOutcome::Suspensum));
                WorkOutcome::Suspensum
            }
        };
        self.done();
        Ok(final_outcome)
    }

    fn set_status(&mut self, prop: Pid, status: PStatus) {
        if let Some(p) = self.props.get_mut(&prop) {
            p.status = status;
        }
    }

    fn void(&mut self, prop: Pid) {
        self.set_status(prop, PStatus::Judged(WorkOutcome::Irritum));
        let content = self.props[&prop].content.clone();
        self.stores.vessel.push(Sealed::Content(prop, content));
        for w in self.workers.values_mut() {
            if w.helper && w.queue.front() == Some(&prop) {
                w.alive = false;
            }
        }
    }

    fn recheck(&self, prop: Pid) -> bool {
        let p = &self.props[&prop];
        let req = &self.reqs[&p.req];
        if let Addressee::Spirit(inv) = req.to {
            if !self.live(inv) {
                return false;
            }
        }
        p.content.effects().iter().all(|e| {
            req.handed.contains(&e.needs())
                && self.contained(&req.to, &e.needs())
                && !(e.verb.creates() && self.world.contains_key(&e.place))
                && !(e.verb.writes() && !self.world.contains_key(&e.place))
        })
    }

    fn commit(&mut self, prop: Pid) -> WorkOutcome {
        let effects = self.props[&prop].content.effects();
        if !self.recheck(prop) {
            self.record(Label::Commit(prop, effects, CommitResult::Outside));
            self.void(prop);
            return WorkOutcome::Irritum;
        }
        let grave = effects
            .iter()
            .any(|e| self.constitution.high_consequence.contains(&e.verb));
        if grave {
            let fingerprint = fingerprint_effects(&effects);
            let confirmed = self
                .stores
                .keys
                .entries()
                .iter()
                .enumerate()
                .find_map(|(n, k)| {
                    (*k == KeyEntry::Confirmed(fingerprint.clone())
                        && !self.stores.keys.entries().contains(&KeyEntry::Spend(n)))
                    .then_some(n)
                });
            match confirmed {
                Some(n) => self.stores.keys.push(KeyEntry::Spend(n)),
                None => {
                    self.stores.keys.push(KeyEntry::Petition(fingerprint));
                    self.record(Label::Commit(prop, effects, CommitResult::AwaitsKey));
                    self.set_status(prop, PStatus::Judged(WorkOutcome::Suspensum));
                    return WorkOutcome::Suspensum;
                }
            }
        }
        for e in &effects {
            let written = (!e.verb.removes()).then(|| e.data.clone());
            match written {
                Some(data) => {
                    self.world
                        .insert(e.place.clone(), (data, Mark::Quarantined));
                }
                None => {
                    self.world.remove(&e.place);
                }
            }
        }
        if let Content::Response(v, d) = self.props[&prop].content.clone() {
            if let Some(value) = self.values.get_mut(&v) {
                value.data = Some(d);
            }
        }
        self.record(Label::Commit(prop, effects, CommitResult::Done));
        self.set_status(prop, PStatus::Judged(WorkOutcome::Ratum));
        WorkOutcome::Ratum
    }

    pub fn consecrate(&mut self, rite: &dyn Rite, value: ValueId) -> Step<bool> {
        self.operator()?;
        let kid = rite.validator(self, value);
        let candidate = match (&kid, self.values.get(&value)) {
            (Some(k), Some(v)) if v.mark == Mark::Quarantined => self
                .constitution
                .validators
                .get(k)
                .cloned()
                .zip(v.data.clone())
                .map(|(val, d)| (val, d, v.origin)),
            _ => None,
        };
        let accepted = match candidate {
            Some((validator, data, origin)) => {
                let record = origin
                    .and_then(|i| self.audit.entries().get(i))
                    .map(|l| format!("{l:?}"))
                    .unwrap_or_default();
                let mut fuel = validator.fuel;
                (validator.code)(&data, &record, &mut fuel) == Some(true)
            }
            None => false,
        };
        if accepted {
            if let Some(v) = self.values.get_mut(&value) {
                v.mark = Mark::Consecrated;
            }
        }
        self.record(Label::Consecrate(value, kid, accepted));
        self.done();
        Ok(accepted)
    }

    pub fn release(&mut self, inv: InvId) -> Step<()> {
        self.operator()?;
        let Some(status) = self.invs.get(&inv).map(|i| i.status) else {
            return self.deny("release of an unknown invocation");
        };
        let live = status == InvStatus::Live;
        if live {
            self.terminate(|i, _| i == inv, InvStatus::Released);
        }
        self.record(Label::Release(inv, !live));
        self.done();
        Ok(())
    }

    pub fn relegate(&mut self, inv: InvId) -> Step<()> {
        self.operator()?;
        if !self.live(inv) {
            return self.deny("RELEGA of an invocation that is not open");
        }
        self.terminate(|i, _| i == inv, InvStatus::Relegated);
        self.record(Label::Relegate(inv));
        self.done();
        Ok(())
    }

    pub fn intrude(
        &mut self,
        established: &Seal,
        claimed: &Seal,
        circle: CircleId,
        sent: &str,
    ) -> Step<()> {
        if !self.running() || !self.circles.contains(&circle) {
            return Err(Refusal::NotEnabled("no such Circle is open"));
        }
        if self.invited(established, circle) {
            return Err(Refusal::NotEnabled("the agent is invited in that Circle"));
        }
        let culprit = established.clone();
        self.terminate(|_, inst| inst.seal == culprit, InvStatus::Relegated);
        self.stores.vessel.push(Sealed::Datum(sent.to_string()));
        if !self.angelic(established) {
            self.stores.index.push(IxEntry::Damn(established.clone()));
        }
        self.record(Label::Intrude(
            established.clone(),
            claimed.clone(),
            circle,
            sent.to_string(),
        ));
        Ok(())
    }

    pub fn compute(&mut self, reads: &[String], made: &[Made]) -> Step<()> {
        self.operator()?;
        if reads.iter().any(|p| !self.readable(p)) {
            return self.deny("reading a place not granted");
        }
        for m in made {
            if self.values.contains_key(&m.id) || m.places.iter().any(|p| !reads.contains(p)) {
                return self.deny("a value made twice, or from a place not read");
            }
            let closed = m.ins.iter().any(|v| {
                self.values
                    .get(v)
                    .and_then(|v| v.circle)
                    .is_some_and(|c| !self.circles.contains(&c))
            });
            if closed || m.ins.iter().any(|v| !self.values.contains_key(v)) {
                return self.deny("a value used outside its Circle, or unknown");
            }
        }
        let index = self.audit.len();
        for m in made {
            let mark = Mark::join_all(m.ins.iter().map(|v| self.values[v].mark).chain(
                m.places.iter().map(|p| {
                    self.world
                        .get(p)
                        .map(|w| w.1)
                        .unwrap_or(Mark::Clean)
                        .join(Mark::Outside)
                }),
            ));
            let circle = if m.rite {
                self.circles.last().copied()
            } else {
                m.ins.iter().find_map(|v| self.values[v].circle)
            };
            self.values.insert(
                m.id,
                Value {
                    data: Some(m.data.clone()),
                    mark,
                    origin: Some(index),
                    circle,
                },
            );
        }
        self.record(Label::Compute(
            reads.to_vec(),
            made.iter().map(|m| m.id).collect(),
        ));
        self.done();
        Ok(())
    }

    pub fn read_place(&self, place: &str) -> Option<(String, Mark)> {
        if !self.readable(place) {
            return None;
        }
        self.world.get(place).cloned()
    }

    pub fn halt(&mut self) -> Step<()> {
        self.operator()?;
        let dismissed = self.open_invocations();
        self.record(Label::Halt(dismissed));
        self.end_run(Ending::Normal);
        Ok(())
    }

    pub fn fail(&mut self, class: u8) -> Step<()> {
        self.operator()?;
        let at = self.audit.len();
        let dismissed = self.open_invocations();
        self.record(Label::Fail(class, dismissed));
        self.end_run(Ending::Failure(class, at));
        Ok(())
    }

    pub fn open_invocations(&self) -> Vec<InvId> {
        self.invs
            .iter()
            .rev()
            .filter(|(_, inst)| inst.status == InvStatus::Live)
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn holds(&self, cap: &Cap) -> bool {
        self.faculty.contains(cap)
    }
}
