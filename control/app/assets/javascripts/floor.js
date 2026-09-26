// The Airlock floor: agents on one side of a wall, main on the other, and
// one door. Every room is a real component; every movement is a real event
// from /floor/events. Drawn in code, in Frameline's paper-and-ink style.

const canvas = document.getElementById("floor");
const ctx = canvas.getContext("2d");
const W = 1280;
const H = 720;
const reducedMotion = matchMedia("(prefers-reduced-motion: reduce)").matches;
const urls = { state: canvas.dataset.state, events: canvas.dataset.events, sessions: canvas.dataset.sessions };
// The data URLs already carry ?repo=…; add parameters properly, never by concatenation.
function withParams(base, params) {
  const u = new URL(base, location.href);
  Object.entries(params).forEach(([k, v]) => u.searchParams.set(k, v));
  return u;
}

// ── Theme ────────────────────────────────────────────────────────────────────

let T = {};
function readTheme() {
  const css = getComputedStyle(document.documentElement);
  const v = (name, fallback) => css.getPropertyValue(name).trim() || fallback;
  const dark = matchMedia("(prefers-color-scheme: dark)").matches;
  T = {
    dark,
    paper: dark ? "#16181d" : "#f4efe4",
    room: dark ? "#1d2027" : "#fbf8f1",
    ink: dark ? "#ddd6c6" : "#26263a",
    muted: v("--muted", "#5f6b7a"),
    line: dark ? "#3a3f4b" : "#d6cfbf",
    accent: v("--accent", "#2f6fd6"),
    good: v("--good", "#1f8a5b"),
    bad: v("--bad", "#c2412d"),
    warn: v("--warn", "#b7791f"),
    skin: dark ? "#e8dcc6" : "#f7e7cf",
    agents: ["#e5487b", "#2f9e8f", "#f2a93b", "#7b61d9", "#3d8bd9", "#d9643d", "#5aa34a", "#b8497e"],
  };
}
readTheme();
matchMedia("(prefers-color-scheme: dark)").addEventListener("change", readTheme);

// ── The floor plan (the architecture) ────────────────────────────────────────

const ROOMS = {
  desks:  { x: 30,   y: 200, w: 370, h: 490, title: "Agents' desks", sub: "Nemotron Super" },
  lab:    { x: 330,  y: 30,  w: 240, h: 150, title: "Sandbox lab", sub: "Token Factory Sandboxes" },
  labels: { x: 470,  y: 318, w: 150, h: 130, title: "Labels & risk", sub: "rules + Nano" },
  review: { x: 650,  y: 40,  w: 270, h: 210, title: "Review desk", sub: "a person" },
  studio: { x: 950,  y: 40,  w: 300, h: 290, title: "Render studio", sub: "policy's render command" },
  queue:  { x: 650,  y: 400, w: 430, h: 130, title: "Merge queue", sub: "batches · test-and-bisect" },
  main:   { x: 1100, y: 360, w: 150, h: 210, title: "main", sub: "only the queue writes here" },
  doctor: { x: 650,  y: 560, w: 300, h: 130, title: "Doctor", sub: "rules + Nemotron Ultra" },
};
const WALL_X = 430;
const DOOR = { y1: 355, y2: 425 };
const AISLE_X = 200;
const RIGHT_AISLE_X = 630;

const DESKS = [
  [110, 262], [290, 262], [110, 372], [290, 372], [110, 482], [290, 482], [110, 592], [290, 592],
];
const seatOf = (i) => { const [x, y] = DESKS[i % DESKS.length]; return { x, y: y + 44 }; };
const LAB_CORRIDOR_Y = 214;
const SPOTS = {
  gateIn: { x: 400, y: 392 },
  labLeft: { x: 395, y: 150 },
  gateOut: { x: 468, y: 392 },
  labRight: { x: 505, y: 132 },
  labels: { x: 530, y: 410 },
  main: { x: 1175, y: 480 },
};
const reviewSlot = (i) => ({ x: 690 + (i % 4) * 48, y: 214 - Math.floor(i / 4) * 30 });
const beltSlot = (i) => ({ x: 700 + (i % 9) * 42, y: 468 });
const doctorSlot = (i) => ({ x: 790 + (i % 4) * 42, y: 652 - Math.floor(i / 4) * 28 });

const STAFF = {
  gate:   { x: 462, y: 480, name: "gate", hat: "cap", color: "#26263a" },
  lab:    { x: 548, y: 142, name: "sandbox", hat: "goggles", color: "#2f9e8f" },
  labels: { x: 588, y: 428, name: "Nano", hat: "visor", color: "#7b61d9" },
  review: { x: 885, y: 168, name: "you", hat: "star", color: "#f2a93b" },
  queue:  { x: 1060, y: 520, name: "queue", hat: "hardhat", color: "#f2c14e" },
  doctor: { x: 700, y: 640, name: "Ultra", hat: "cross", color: "#ffffff" },
  studio: { x: 985, y: 300, name: "render", hat: "beret", color: "#e5487b" },
};

const EXPLAIN = {
  desks: ["Agents' desks", "Each agent is a worker in the control service. It asks Nemotron Super for SEARCH/REPLACE edits, applies them inside its own clone and runs the task's check. The model never runs a command and never touches git; a task that cannot pass stops at its budget and pushes nothing."],
  lab: ["Sandbox lab", "Checks and evidence run in Token Factory Sandboxes: a clean copy of the committed tree, no network, a fixed image. After a push, Airlock re-runs the command the agent cites in its Airlock-Evidence trailer, so \"I tested it\" becomes something anyone can repeat."],
  gate: ["The gate", "The only door to main. A pre-receive hook sends every push to the gate, which checks protected refs, the agent's own branch namespace, forbidden paths, secrets, skipped or deleted tests, and a well-formed evidence trailer with an allowed command. A refused push never lands."],
  labels: ["Labels and risk", "Rules label what they can read from the diff; Nemotron Nano votes three times on the rest, from a closed vocabulary. A noisy-OR turns the labels and the agent's track record into a risk score that decides where the change goes next."],
  review: ["The review desk", "Only some changes reach a person: sensitive paths, changed golden images, or risk above a threshold that adapts so the reviewer's load stays under a target. Visual changes arrive with before/after boards and animatics; a rejection's reason goes back to the next agent."],
  queue: ["Merge queue", "Changes are integrated in batches on top of main. A red batch is split in halves until the broken change is isolated, and the batch size minimizes CI runs for the agents' current failure rate."],
  main: ["main", "Only the merge queue updates main, and it does so through the same gate. Every green batch is re-rendered in the studio."],
  doctor: ["The doctor", "When the queue fails a change, rules name the category (conflict, compile error, failing test, visual regression, a change that only fails with others) and Nemotron Ultra names the culprit file and the next step. Agents retry with the diagnosis; a person decides the rest."],
  studio: ["Render studio", "The policy's render command, never an agent's, draws the boards and the animatic of main through the same runner as CI. The Frameline tab shows them next to the brief they came from."],
};
const TOUR = ["desks", "lab", "gate", "labels", "review", "queue", "doctor", "main", "studio"];

// ── Entities ─────────────────────────────────────────────────────────────────

const SPEED = { agent: 150, parcel: 190 };
let sandboxOn = false;
const agents = new Map();
const parcels = new Map();
const flashes = {};          // station -> {color, until, text}
const counters = { merged: 0 };
let screen = null;           // {img, sha, rendering}
let beltRunning = 0;         // until timestamp
let beltLamp = null;         // {color, until}
let now = performance.now();

function hashColor(name) {
  let h = 0;
  for (const c of name) h = (h * 31 + c.charCodeAt(0)) >>> 0;
  return T.agents[h % T.agents.length];
}

class Mover {
  constructor(x, y, speed) { this.x = x; this.y = y; this.speed = speed; this.queue = []; this.walking = false; }
  moveTo(points) { this.queue.push({ type: "move", points: points.map((p) => ({ ...p })) }); return this; }
  then(fn) { this.queue.push({ type: "call", fn }); return this; }
  wait(ms) { this.queue.push({ type: "wait", until: null, ms }); return this; }
  // Hold every later action until cond() is true (a carried card waits for the gate).
  until(cond) { this.queue.push({ type: "until", cond }); return this; }
  busy() { return this.queue.length > 0; }
  update(dt) {
    this.walking = false;
    // Fall behind in live mode? Catch up rather than lag the log.
    const boost = 1 + Math.min(this.queue.length, 8) * 0.35;
    while (this.queue.length) {
      const a = this.queue[0];
      if (a.type === "call") { this.queue.shift(); a.fn(); continue; }
      if (a.type === "until") { if (!a.cond()) return; this.queue.shift(); continue; }
      if (a.type === "wait") {
        if (a.until === null) a.until = now + (reducedMotion ? 0 : a.ms / boost);
        if (now < a.until) return;
        this.queue.shift(); continue;
      }
      const target = a.points[0];
      if (!target) { this.queue.shift(); continue; }
      const dx = target.x - this.x, dy = target.y - this.y, d = Math.hypot(dx, dy);
      const step = reducedMotion ? Infinity : this.speed * boost * dt;
      if (d <= step) { this.x = target.x; this.y = target.y; a.points.shift(); if (!a.points.length) this.queue.shift(); continue; }
      this.x += (dx / d) * step; this.y += (dy / d) * step; this.walking = true;
      return;
    }
  }
}

class Agent extends Mover {
  constructor(name) {
    const desk = agents.size;
    const seat = seatOf(desk);
    super(seat.x, seat.y, SPEED.agent);
    this.name = name; this.desk = desk; this.color = hashColor(name);
    this.mode = "idle"; this.bubble = null; this.carrying = null; this.phase = Math.random() * 6;
    this.rogue = false;
  }
  get seat() { return seatOf(this.desk); }
  // Agents stay on their side of the wall: desk ⇄ aisle ⇄ gate or lab.
  routeTo(spot) {
    const pts = [{ x: AISLE_X, y: this.y }];
    if (spot === SPOTS.labLeft) pts.push({ x: AISLE_X, y: LAB_CORRIDOR_Y }, { x: 380, y: LAB_CORRIDOR_Y }, spot);
    else pts.push({ x: AISLE_X, y: spot.y }, spot);
    if (Math.abs(this.x - AISLE_X) < 2) pts.shift();
    return this.moveTo(pts);
  }
  goDesk() {
    const s = this.seat;
    const pts = [];
    if (this.y < LAB_CORRIDOR_Y - 5) pts.push({ x: 380, y: LAB_CORRIDOR_Y }, { x: AISLE_X, y: LAB_CORRIDOR_Y });
    else pts.push({ x: AISLE_X, y: this.y });
    pts.push({ x: AISLE_X, y: s.y }, s);
    return this.moveTo(pts);
  }
  say(text, color = null, ms = 4200) { return this.then(() => { this.bubble = { text, color, until: now + ms }; }); }
}

class Parcel extends Mover {
  constructor(key, task, agent, x, y) {
    super(x, y, SPEED.parcel);
    this.key = key; this.task = task || "?"; this.agent = agent; this.stamp = null; this.alpha = 1; this.fading = false;
    this.label = /^[A-Z]+-\d+$/.test(this.task) ? this.task : this.task.split(/[-_]/)[0].slice(0, 6);
    this.carriedBy = null;
    this.station = null; this.color = agent ? hashColor(agent) : T.muted;
  }
  // Parcels move on main's side: along the right aisle between stations.
  routeTo(target, station) {
    const pts = [];
    if (this.x < RIGHT_AISLE_X - 5 || Math.abs(this.x - target.x) > 4) {
      pts.push({ x: RIGHT_AISLE_X, y: this.y }, { x: RIGHT_AISLE_X, y: target.y });
    }
    pts.push(target);
    this.station = station;
    return this.moveTo(pts);
  }
  stampWith(text, color) { return this.then(() => { this.stamp = { text, color }; }); }
  vanish(ms = 1200) { return this.then(() => { this.fading = true; this.fadeFrom = now; this.fadeMs = reducedMotion ? 1 : ms; }); }
}

function agentFor(name) {
  if (!name) return null;
  if (!agents.has(name)) agents.set(name, new Agent(name));
  return agents.get(name);
}

function parcelKey(ev) { return ev.change ? `c${ev.change}` : `t${ev.task}:${ev.actor}`; }
function findParcel(ev) {
  if (ev.change && parcels.has(`c${ev.change}`)) return parcels.get(`c${ev.change}`);
  const loose = parcels.get(`t${ev.task}:${ev.actor}`);
  if (loose && ev.change) { parcels.delete(loose.key); loose.key = `c${ev.change}`; parcels.set(loose.key, loose); }
  return loose || null;
}
function newParcel(ev, at) {
  const p = new Parcel(parcelKey(ev), ev.task, ev.actor, at.x, at.y);
  parcels.set(p.key, p);
  return p;
}
function stationCount(station, except) {
  let n = 0;
  for (const p of parcels.values()) if (p !== except && p.station === station && !p.fading) n += 1;
  return n;
}
// The card exists from the moment of the push, so every later event finds it;
// it rides with its agent and does nothing until the gate lets go of it.
function carry(agent, ev) {
  const key = `t${ev.task}:${ev.actor}`;
  parcels.delete(key);
  const p = new Parcel(key, ev.task, ev.actor, agent.x + 14, agent.y - 18);
  parcels.set(key, p);
  p.carriedBy = agent; agent.carrying = p;
  p.until(() => !p.carriedBy);
  return p;
}

function flash(station, color, text = null, ms = 1600) { flashes[station] = { color, text, until: now + ms }; }

// ── Events → movement ────────────────────────────────────────────────────────

const LOG_MAX = 14;
function log(ev, text) {
  const list = document.getElementById("floor-log");
  const li = document.createElement("li");
  const t = new Date(ev.at);
  li.innerHTML = `<time>${t.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", second: "2-digit" })}</time> `;
  li.append(document.createTextNode(text));
  li.dataset.kind = ev.kind.split(".")[0];
  list.prepend(li);
  while (list.children.length > LOG_MAX) list.lastChild.remove();
}

const short = (s, n = 70) => (s && s.length > n ? `${s.slice(0, n - 1)}…` : s || "");

// What each event means, in one line, for the log (and for screen readers).
function describe(ev) {
  const d = ev.data || {};
  const n = (d.changes || []).length;
  switch (ev.kind) {
    case "agent.started": return `${ev.actor} picks up ${ev.task}: ${ev.text}`;
    case "agent.thinking": return `${ev.actor} asks Nemotron Super (attempt ${d.attempt})`;
    case "agent.editing": return `${ev.actor} edits ${(d.files || []).join(", ")}`;
    case "agent.edits_refused": return `${ev.actor}'s edits were refused: ${short(ev.text, 90)}`;
    case "agent.goldens": return `${ev.actor} regenerates the golden images`;
    case "agent.testing": return `${ev.actor} runs ${ev.text}${d.sandbox ? " in a sandbox" : " (local runner)"}`;
    case "agent.check_failed": return `${ev.actor}'s check failed: ${short(ev.text, 90)}`;
    case "agent.pushing": return `${ev.actor} pushes ${ev.text}`;
    case "agent.finished": {
      const verb = d.status === "pushed" ? "finished" : d.status === "gave_up" ? "gave up on" : "hit an error on";
      return `${ev.actor} ${verb} ${ev.task}${d.attempts ? ` after ${d.attempts} attempt(s)` : ""}`;
    }
    case "gate.accepted": return `gate accepts ${ev.actor}'s push (${ev.task})`;
    case "gate.rejected": return `gate refuses ${ev.actor}: ${ev.text}`;
    case "evidence.skipped": return `evidence not re-run for ${ev.task}: no sandbox configured`;
    case "evidence.verified": return `evidence reproduced in a sandbox for ${ev.task}`;
    case "evidence.failed": return `evidence did not reproduce for ${ev.task}; rejected`;
    case "change.routed": return d.state === "needs_review" ? `${ev.task} goes to a person: ${short(ev.text, 80)}` : `${ev.task} goes to the merge queue (risk ${d.risk ?? "?"})`;
    case "review.approved": return `${ev.actor} approves ${ev.task}`;
    case "review.rejected": return `${ev.actor} rejects ${ev.task}: ${ev.text}`;
    case "queue.batch_started": return `merge queue tests a batch of ${n} on top of main`;
    case "queue.ci_run": return `CI run ${d.run} on ${n} change(s): ${d.ok ? "green" : "red, splitting the batch"}`;
    case "queue.batch_finished": return `batch done: ${ev.text} in ${d.runs} CI run(s)`;
    case "queue.recovered": return "the queue recovered a batch left by a stopped process";
    case "change.merged": return `${ev.task} is on main (${ev.text})`;
    case "change.failed": return `${ev.task} failed in the queue (${ev.text}); to the doctor`;
    case "diagnosis.done": return `diagnosis for ${ev.task}: ${d.category}, next: ${d.next_step}. ${short(ev.text, 90)}`;
    case "render.started": return `rendering ${ev.text}`;
    case "render.finished": return d.ok ? `rendered ${d.sha?.slice(0, 7)}: ${ev.text}` : `render failed for ${d.sha?.slice(0, 7)}`;
    default: return ev.kind;
  }
}

function apply(ev) {
  log(ev, describe(ev));
  animate(ev);
}

function animate(ev) {
  const a = ev.actor && !["airlock"].includes(ev.actor) && ev.kind.startsWith("agent.") ? agentFor(ev.actor) : null;
  switch (ev.kind) {
    case "agent.started":
      a.goDesk().say(`${ev.task}: ${short(ev.text, 44)}`);
      break;
    case "agent.thinking":
      a.goDesk().then(() => { a.mode = "thinking"; }).say(`asking ${short(ev.text?.replace("asking ", ""), 30)}…`, null, 3000);
      break;
    case "agent.editing":
      a.goDesk().then(() => { a.mode = "typing"; }).say(short(ev.text, 60), null, 3200);
      break;
    case "agent.edits_refused":
      a.say("edits refused", T.bad, 2600).then(() => { a.mode = "idle"; });
      break;
    case "agent.goldens":
      a.routeTo(SPOTS.labLeft).then(() => { a.mode = "testing"; flash("lab", T.accent, "golden images"); }).wait(900);
      break;
    case "agent.testing":
      a.routeTo(SPOTS.labLeft).then(() => { a.mode = "testing"; flash("lab", T.accent, "running", 2400); }).wait(1200);
      break;
    case "agent.check_failed":
      a.then(() => flash("lab", T.bad, "red")).say(short(ev.text, 50) || "check failed", T.bad, 3000).goDesk()
        .then(() => { a.mode = "idle"; });
      break;
    case "agent.pushing": {
      carry(a, ev);
      a.goDesk().routeTo(SPOTS.gateIn).say("push", null, 1500);
      break;
    }
    case "agent.finished": {
      const status = ev.data.status;
      a.goDesk().then(() => { a.mode = status === "pushed" ? "idle" : "slump"; })
        .say(status === "pushed" ? `${ev.task} done` : status === "gave_up" ? `gave up on ${ev.task}` : `${ev.task}: error`,
             status === "pushed" ? T.good : T.bad, 4000);
      break;
    }
    case "gate.accepted":
    case "gate.rejected": {
      const ag = agentFor(ev.actor);
      const ok = ev.kind === "gate.accepted";
      // A push we did not see leave a desk (a scripted agent): it walks to the door now.
      let p = findParcel(ev);
      if (!p || !p.carriedBy) { p = carry(ag, ev); ag.goDesk().routeTo(SPOTS.gateIn); }
      ag.then(() => {
        flash("gate", ok ? T.good : T.bad, ok ? "accepted" : "refused");
        p.carriedBy = null; ag.carrying = null; p.x = SPOTS.gateIn.x; p.y = SPOTS.gateIn.y;
        if (!ok) ag.rogue = true;
      });
      if (ok) p.moveTo([SPOTS.gateOut]).then(() => { p.station = "gate"; });
      else p.stampWith("✕", T.bad).wait(700).vanish(900);
      if (!ok) ag.say(short(ev.text, 58), T.bad, 4200);
      ag.goDesk();
      break;
    }
    case "evidence.verified":
    case "evidence.failed":
    case "evidence.skipped": {
      const p = findParcel(ev) || newParcel(ev, SPOTS.gateOut);
      const status = ev.kind.split(".")[1];
      p.moveTo([{ x: 520, y: SPOTS.gateOut.y }, { x: 520, y: 185 }, SPOTS.labRight]).then(() => flash("lab", status === "failed" ? T.bad : status === "verified" ? T.good : T.muted, `evidence ${status}`))
        .stampWith(status === "verified" ? "✓" : status === "failed" ? "✕" : "–", status === "verified" ? T.good : status === "failed" ? T.bad : T.muted)
        .wait(600);
      if (status === "failed") p.vanish();
      else p.moveTo([{ x: 520, y: 185 }, { x: 520, y: SPOTS.labels.y }, SPOTS.labels]);
      break;
    }
    case "change.routed": {
      const p = findParcel(ev) || newParcel(ev, SPOTS.gateOut);
      if (p.station !== "labels") p.moveTo([{ x: 520, y: SPOTS.gateOut.y }, SPOTS.labels]);
      p.then(() => flash("labels", T.accent, ev.data.risk != null ? `risk ${ev.data.risk}` : "labelled")).wait(700);
      p.station = "labels";
      if (ev.data.state === "needs_review") {
        p.routeTo(reviewSlot(stationCount("review", p)), "review").stampWith("?", T.warn);
      } else {
        p.routeTo(beltSlot(stationCount("queue", p)), "queue");
      }
      break;
    }
    case "review.approved":
    case "review.rejected": {
      const p = findParcel(ev) || newParcel(ev, reviewSlot(0));
      const ok = ev.kind === "review.approved";
      p.then(() => flash("review", ok ? T.good : T.bad, ok ? "approved" : "rejected")).stampWith(ok ? "✓" : "✕", ok ? T.good : T.bad).wait(500);
      if (ok) p.routeTo(beltSlot(stationCount("queue", p)), "queue");
      else {
        p.routeTo(SPOTS.gateOut, "returned").vanish();
        const ag = agentFor(ev.data.agent);
        if (ag) ag.say(`reviewer: ${short(ev.text, 50)}`, T.warn, 5000);
      }
      break;
    }
    case "queue.batch_started":
      beltRunning = now + 60_000;
      (ev.data.changes || []).forEach((id, i) => {
        const p = parcels.get(`c${id}`) || newParcel({ ...ev, change: id, task: ev.data.tasks?.[i] }, beltSlot(i));
        p.routeTo(beltSlot(i), "queue");
      });
      flash("queue", T.accent, `batch of ${(ev.data.changes || []).length}`);
      break;
    case "queue.ci_run":
      beltLamp = { color: ev.data.ok ? T.good : T.bad, until: now + 2200 };
      flash("queue", ev.data.ok ? T.good : T.bad, ev.data.ok ? `run ${ev.data.run}: green` : `run ${ev.data.run}: red → bisect`);
      break;
    case "queue.batch_finished":
      beltRunning = now + 800;
      break;
    case "queue.recovered":
      flash("queue", T.warn, "recovered");
      break;
    case "change.merged": {
      const p = findParcel(ev) || newParcel(ev, beltSlot(0));
      p.moveTo([{ x: p.x, y: 468 }, { x: 1085, y: 468 }, SPOTS.main]).then(() => { counters.merged += 1; flash("main", T.good, `+ ${ev.task}`); })
        .vanish(700);
      p.station = "main";
      break;
    }
    case "change.failed": {
      const p = findParcel(ev) || newParcel(ev, beltSlot(0));
      p.routeTo(doctorSlot(stationCount("doctor", p)), "doctor").stampWith("✕", T.bad);
      break;
    }
    case "diagnosis.done": {
      const p = findParcel(ev);
      flash("doctor", T.accent, `${ev.data.category} → ${String(ev.data.next_step || "").replace("_", " ")}`, 5000);
      if (p) p.stampWith("Dx", T.accent);
      break;
    }
    case "render.started":
      screen = { ...(screen || {}), rendering: true };
      break;
    case "render.finished": {
      const board = (ev.data.files || []).find((f) => f.endsWith(".png"));
      screen = { ...(screen || {}), rendering: false };
      if (board && ev.data.ok) loadScreen(ev.data.sha, board);
      break;
    }
    default:
      return;
  }
}

function loadScreen(sha, file) {
  const img = new Image();
  img.onload = () => { screen = { ...(screen || {}), img, sha }; };
  img.src = `/frameline/renders/${sha}/${file}`;
}

// ── Drawing ──────────────────────────────────────────────────────────────────

// Hand-drawn ink: deterministic jitter so lines do not shimmer between frames.
function jitter(seed) { const s = Math.sin(seed * 12.9898) * 43758.5453; return (s - Math.floor(s)) - 0.5; }
function inkRect(r, seed, fill) {
  const { x, y, w, h } = r;
  const j = (i) => jitter(seed + i) * 2.2;
  ctx.beginPath();
  ctx.moveTo(x + j(1), y + j(2));
  ctx.lineTo(x + w + j(3), y + j(4));
  ctx.lineTo(x + w + j(5), y + h + j(6));
  ctx.lineTo(x + j(7), y + h + j(8));
  ctx.closePath();
  if (fill) { ctx.fillStyle = fill; ctx.fill(); }
  ctx.strokeStyle = T.ink; ctx.lineWidth = 1.6; ctx.stroke();
}
function text(str, x, y, { size = 12, weight = 500, color = T.ink, align = "left", font = "system-ui, sans-serif" } = {}) {
  ctx.font = `${weight} ${size}px ${font}`; ctx.fillStyle = color; ctx.textAlign = align; ctx.textBaseline = "alphabetic";
  ctx.fillText(str, x, y);
}
function roundRect(x, y, w, h, r, fill, stroke) {
  ctx.beginPath(); ctx.roundRect(x, y, w, h, r);
  if (fill) { ctx.fillStyle = fill; ctx.fill(); }
  if (stroke) { ctx.strokeStyle = stroke; ctx.lineWidth = 1.4; ctx.stroke(); }
}

function drawPaper() {
  ctx.fillStyle = T.paper; ctx.fillRect(0, 0, W, H);
  // Paper grain: sparse fixed dots.
  ctx.fillStyle = T.dark ? "rgba(255,255,255,0.025)" : "rgba(60,40,10,0.04)";
  for (let i = 0; i < 900; i += 1) {
    const x = (jitter(i) + 0.5) * W, y = (jitter(i + 7000) + 0.5) * H;
    ctx.fillRect(x, y, 1.4, 1.4);
  }
}

function drawRoom(id, seed) {
  const r = ROOMS[id];
  const f = flashes[id];
  const active = f && now < f.until;
  inkRect(r, seed, active ? mix(T.room, f.color, 0.12) : T.room);
  text(r.title.toUpperCase(), r.x + 10, r.y + 18, { size: 11, weight: 700, color: T.ink });
  text(r.sub, r.x + 10, r.y + 32, { size: 10.5, color: T.muted });
  if (active && f.text) {
    const tw = Math.min(ctx.measureText(f.text).width + 14, r.w - 16);
    roundRect(r.x + r.w - tw - 8, r.y + 8, tw, 20, 10, f.color);
    text(short(f.text, 38), r.x + r.w - tw / 2 - 8, r.y + 22, { size: 10.5, weight: 700, color: "#fff", align: "center" });
  }
}

function mix(a, b, t) {
  const p = (c) => [1, 3, 5].map((i) => parseInt(c.slice(i, i + 2), 16));
  if (!/^#[0-9a-f]{6}$/i.test(a) || !/^#[0-9a-f]{6}$/i.test(b)) return a;
  const [x, y] = [p(a), p(b)];
  return `rgb(${x.map((v, i) => Math.round(v + (y[i] - v) * t)).join(",")})`;
}

function drawWall() {
  ctx.strokeStyle = T.ink; ctx.lineWidth = 7; ctx.lineCap = "round";
  ctx.beginPath(); ctx.moveTo(WALL_X, 180); ctx.lineTo(WALL_X, DOOR.y1);
  ctx.moveTo(WALL_X, DOOR.y2); ctx.lineTo(WALL_X, 700); ctx.stroke();
  // The door: two leaves that swing open while a push is inspected.
  const f = flashes.gate; const open = f && now < f.until;
  const color = open ? f.color : T.muted;
  ctx.lineWidth = 3; ctx.strokeStyle = color;
  const swing = open ? 0.9 : 0.08;
  ctx.beginPath();
  ctx.moveTo(WALL_X, DOOR.y1); ctx.lineTo(WALL_X + Math.sin(swing) * 32, DOOR.y1 + Math.cos(swing) * 32);
  ctx.moveTo(WALL_X, DOOR.y2); ctx.lineTo(WALL_X + Math.sin(swing) * 32, DOOR.y2 - Math.cos(swing) * 32);
  ctx.stroke();
  text("THE GATE", WALL_X - 12, DOOR.y1 - 8, { size: 11, weight: 800, align: "right" });
  text("pre-receive hook", WALL_X - 12, DOOR.y1 + 6, { size: 10, color: T.muted, align: "right" });
  if (open && f.text) {
    roundRect(WALL_X - 82, DOOR.y2 + 8, 70, 18, 9, f.color);
    text(f.text, WALL_X - 47, DOOR.y2 + 21, { size: 10, weight: 700, color: "#fff", align: "center" });
  }
  text("AGENTS' SIDE", 40, 190, { size: 10, weight: 700, color: T.muted });
  text("MAIN'S SIDE", 450, 305, { size: 10, weight: 700, color: T.muted });
}

function drawDesks() {
  DESKS.forEach(([x, y], i) => {
    const owner = [...agents.values()].find((a) => a.desk === i);
    inkRect({ x: x - 42, y: y - 10, w: 84, h: 34 }, 300 + i, T.dark ? "#2a2d35" : "#efe6d4");
    // Monitor: lights up while its agent types or thinks.
    const lit = owner && ["typing", "thinking"].includes(owner.mode) && Math.hypot(owner.x - owner.seat.x, owner.y - owner.seat.y) < 4;
    roundRect(x - 16, y - 4, 32, 20, 3, lit ? mix(T.room, owner.color, 0.55) : (T.dark ? "#3a3f4b" : "#d9d0bd"), T.ink);
    if (lit && owner.mode === "typing" && !reducedMotion) {
      for (let k = 0; k < 3; k += 1) {
        const w = 8 + ((Math.floor(now / 180) + k * 3 + i) % 5) * 3;
        ctx.fillStyle = "rgba(255,255,255,0.8)"; ctx.fillRect(x - 12, y + k * 5, w, 2);
      }
    }
    if (owner) text(owner.name, x, y + 88, { size: 10.5, weight: 600, color: owner.rogue ? T.bad : T.muted, align: "center" });
  });
}

function drawLab() {
  const r = ROOMS.lab;
  for (let i = 0; i < 3; i += 1) {
    const bx = r.x + 30 + i * 70;
    roundRect(bx, r.y + 62, 48, 16, 3, T.dark ? "#2a2d35" : "#e8e0cf", T.ink);
    ctx.fillStyle = i === 1 ? "#2f9e8f" : "#7b61d9"; ctx.fillRect(bx + 8, r.y + 50, 6, 12); ctx.fillRect(bx + 30, r.y + 46, 6, 16);
  }
  const f = flashes.lab; const on = f && now < f.until;
  ctx.beginPath(); ctx.arc(r.x + r.w - 20, r.y + 20, 7, 0, Math.PI * 2);
  ctx.fillStyle = on ? f.color : T.line; ctx.fill(); ctx.strokeStyle = T.ink; ctx.lineWidth = 1.2; ctx.stroke();
  text(sandboxOn ? "no network · clean tree" : "local runner (development)", r.x + 10, r.y + r.h - 12,
       { size: 10, color: sandboxOn ? T.muted : T.warn, weight: sandboxOn ? 500 : 700 });
}

function drawReview() {
  const r = ROOMS.review;
  inkRect({ x: r.x + 120, y: r.y + 64, w: 96, h: 40 }, 41, T.dark ? "#2a2d35" : "#efe6d4");
  text(`${stationCount("review")} waiting`, r.x + 10, r.y + r.h - 12, { size: 11, color: stationCount("review") ? T.warn : T.muted, weight: 700 });
}

function drawBelt() {
  const r = ROOMS.queue;
  const y = 452;
  roundRect(r.x + 20, y, r.w - 70, 34, 6, T.dark ? "#2a2d35" : "#e2d8c4", T.ink);
  const moving = now < beltRunning && !reducedMotion;
  const off = moving ? (now / 40) % 20 : 0;
  ctx.strokeStyle = T.line; ctx.lineWidth = 2;
  for (let x = r.x + 26 + off; x < r.x + r.w - 56; x += 20) { ctx.beginPath(); ctx.moveTo(x, y + 4); ctx.lineTo(x - 8, y + 30); ctx.stroke(); }
  const lampOn = beltLamp && now < beltLamp.until;
  ctx.beginPath(); ctx.arc(r.x + r.w - 30, r.y + 22, 8, 0, Math.PI * 2);
  ctx.fillStyle = lampOn ? beltLamp.color : T.line; ctx.fill(); ctx.strokeStyle = T.ink; ctx.lineWidth = 1.2; ctx.stroke();
  text("CI", r.x + r.w - 30, r.y + 42, { size: 9.5, color: T.muted, align: "center", weight: 700 });
}

function drawMain() {
  const r = ROOMS.main;
  const cx = r.x + r.w / 2, cy = r.y + 105;
  ctx.beginPath(); ctx.arc(cx, cy, 44, 0, Math.PI * 2);
  ctx.fillStyle = T.dark ? "#2a2d35" : "#e8e0cf"; ctx.fill(); ctx.strokeStyle = T.ink; ctx.lineWidth = 2; ctx.stroke();
  ctx.beginPath(); ctx.arc(cx, cy, 30, 0, Math.PI * 2); ctx.stroke();
  const spin = reducedMotion ? 0 : (flashes.main && now < flashes.main.until ? now / 150 : 0);
  for (let k = 0; k < 6; k += 1) {
    const a = spin + (k * Math.PI) / 3;
    ctx.beginPath(); ctx.moveTo(cx + Math.cos(a) * 10, cy + Math.sin(a) * 10); ctx.lineTo(cx + Math.cos(a) * 28, cy + Math.sin(a) * 28); ctx.stroke();
  }
  text(`${counters.merged} merged`, cx, r.y + r.h - 14, { size: 11, weight: 700, color: T.good, align: "center" });
}

function drawStudio() {
  const r = ROOMS.studio;
  const sx = r.x + 20, sy = r.y + 46, sw = r.w - 40, sh = 170;
  roundRect(sx, sy, sw, sh, 4, T.dark ? "#0e0f12" : "#fffdf8", T.ink);
  if (screen?.img) {
    const s = Math.min((sw - 8) / screen.img.width, (sh - 8) / screen.img.height);
    const iw = screen.img.width * s, ih = screen.img.height * s;
    ctx.drawImage(screen.img, sx + (sw - iw) / 2, sy + (sh - ih) / 2, iw, ih);
    text(`main ${screen.sha?.slice(0, 7) || ""}`, sx + sw - 6, sy + sh + 14, { size: 10, color: T.muted, align: "right" });
  } else {
    text("the latest board of main appears here", sx + sw / 2, sy + sh / 2, { size: 11, color: T.muted, align: "center" });
  }
  if (screen?.rendering && !reducedMotion) {
    ctx.fillStyle = "rgba(242,193,78,0.18)";
    ctx.beginPath(); ctx.moveTo(STAFF.studio.x + 14, STAFF.studio.y - 22); ctx.lineTo(sx + 10, sy + sh); ctx.lineTo(sx + sw - 10, sy + sh); ctx.fill();
    text("rendering…", sx + sw - 6, sy - 6, { size: 10, color: T.warn, weight: 700, align: "right" });
  }
}

function drawPerson(x, y, color, { walking = false, phase = 0, hat = null, slump = false, outline = T.ink, scale = 1 } = {}) {
  ctx.save(); ctx.translate(x, y); ctx.scale(scale, scale);
  ctx.fillStyle = T.dark ? "rgba(0,0,0,0.35)" : "rgba(40,30,10,0.12)";
  ctx.beginPath(); ctx.ellipse(0, 2, 11, 4, 0, 0, Math.PI * 2); ctx.fill();
  const swing = walking && !reducedMotion ? Math.sin(phase) * 5 : 0;
  ctx.strokeStyle = outline; ctx.lineWidth = 2.2; ctx.lineCap = "round";
  ctx.beginPath(); ctx.moveTo(-4, -8); ctx.lineTo(-4 + swing, 0); ctx.moveTo(4, -8); ctx.lineTo(4 - swing, 0); ctx.stroke();
  const lean = slump ? 0.25 : 0;
  ctx.rotate(lean);
  roundRect(-9, -26, 18, 20, 7, color, outline);
  ctx.beginPath(); ctx.arc(0, -33, 7.5, 0, Math.PI * 2); ctx.fillStyle = T.skin; ctx.fill(); ctx.lineWidth = 1.6; ctx.stroke();
  ctx.fillStyle = T.ink; ctx.fillRect(-3.2, -34, 1.6, 1.8); ctx.fillRect(1.8, -34, 1.6, 1.8);
  ctx.lineWidth = 1.4;
  switch (hat) {
    case "cap": ctx.fillStyle = T.ink; ctx.beginPath(); ctx.arc(0, -37, 7.5, Math.PI, 0); ctx.fill(); ctx.fillRect(0, -38, 11, 2.5); break;
    case "visor": ctx.fillStyle = "#7b61d9"; ctx.fillRect(-8, -41, 16, 3); break;
    case "hardhat": ctx.fillStyle = "#f2c14e"; ctx.beginPath(); ctx.arc(0, -36, 8.5, Math.PI, 0); ctx.fill(); ctx.stroke(); break;
    case "goggles": ctx.strokeStyle = "#2f9e8f"; ctx.lineWidth = 2; ctx.strokeRect(-6, -36, 12, 4); break;
    case "cross": ctx.fillStyle = T.bad; ctx.fillRect(-1.5, -22, 3, 10); ctx.fillRect(-5, -18.5, 10, 3); break;
    case "beret": ctx.fillStyle = "#e5487b"; ctx.beginPath(); ctx.ellipse(1, -40, 8, 3.5, -0.2, 0, Math.PI * 2); ctx.fill(); break;
    case "star": {
      ctx.fillStyle = "#f2c14e"; ctx.beginPath();
      for (let k = 0; k < 10; k += 1) { const rr = k % 2 ? 2.5 : 6; const a = -Math.PI / 2 + (k * Math.PI) / 5; ctx.lineTo(Math.cos(a) * rr, -48 + Math.sin(a) * rr); }
      ctx.closePath(); ctx.fill(); ctx.stroke(); break;
    }
    default: break;
  }
  ctx.restore();
}

function drawBubble(x, y, str, color) {
  ctx.font = "600 11px system-ui, sans-serif";
  const words = str.split(" "); const lines = []; let line = "";
  for (const w of words) { const t = line ? `${line} ${w}` : w; if (ctx.measureText(t).width > 170 && line) { lines.push(line); line = w; } else line = t; }
  if (line) lines.push(line);
  const shown = lines.slice(0, 2);
  const bw = Math.max(...shown.map((l) => ctx.measureText(l).width)) + 16, bh = shown.length * 14 + 10;
  const bx = Math.min(Math.max(x - bw / 2, 6), W - bw - 6), by = y - bh - 52;
  roundRect(bx, by, bw, bh, 8, T.room, color || T.ink);
  ctx.beginPath(); ctx.moveTo(x - 5, by + bh); ctx.lineTo(x, by + bh + 7); ctx.lineTo(x + 5, by + bh); ctx.fillStyle = T.room; ctx.fill();
  shown.forEach((l, i) => text(l, bx + 8, by + 17 + i * 14, { size: 11, weight: 600, color: color || T.ink }));
}

function drawParcel(p) {
  let alpha = p.alpha;
  if (p.fading) alpha = Math.max(0, 1 - (now - p.fadeFrom) / p.fadeMs);
  if (alpha <= 0) { parcels.delete(p.key); return; }
  ctx.save(); ctx.globalAlpha = alpha;
  ctx.font = "800 9.5px system-ui, sans-serif";
  const hw = Math.max(34, ctx.measureText(p.label).width + 10) / 2;
  roundRect(p.x - hw, p.y - 11, hw * 2, 22, 3, T.room, p.color);
  ctx.fillStyle = p.color; ctx.fillRect(p.x - hw, p.y - 11, hw * 2, 4);
  text(p.label, p.x, p.y + 6, { size: 9.5, weight: 800, align: "center" });
  if (p.stamp) {
    ctx.beginPath(); ctx.arc(p.x + hw - 2, p.y - 10, 7.5, 0, Math.PI * 2); ctx.fillStyle = p.stamp.color; ctx.fill();
    text(p.stamp.text, p.x + hw - 2, p.y - 6.5, { size: 9, weight: 800, color: "#fff", align: "center" });
  }
  ctx.restore();
}

// ── Explain mode ─────────────────────────────────────────────────────────────

let mode = "live";
let selected = null;
function drawExplain() {
  const order = TOUR;
  ctx.save();
  ctx.fillStyle = T.dark ? "rgba(10,12,16,0.35)" : "rgba(244,239,228,0.35)";
  ctx.fillRect(0, 0, W, H);
  order.forEach((id, i) => {
    const r = id === "gate" ? { x: WALL_X - 40, y: DOOR.y1 - 30, w: 80, h: DOOR.y2 - DOOR.y1 + 60 } : ROOMS[id];
    const on = selected === id;
    ctx.lineWidth = on ? 4 : 2; ctx.strokeStyle = on ? T.accent : T.muted; ctx.setLineDash(on ? [] : [6, 5]);
    ctx.strokeRect(r.x - 4, r.y - 4, r.w + 8, r.h + 8); ctx.setLineDash([]);
    ctx.beginPath(); ctx.arc(r.x - 6, r.y - 6, 12, 0, Math.PI * 2); ctx.fillStyle = on ? T.accent : T.ink; ctx.fill();
    text(String(i + 1), r.x - 6, r.y - 1.5, { size: 12, weight: 800, color: T.paper, align: "center" });
  });
  // The journey of a change, drawn once: desk → lab → gate → labels → review or queue → main.
  ctx.strokeStyle = T.accent; ctx.lineWidth = 2.5; ctx.setLineDash([10, 6]);
  ctx.beginPath();
  [[seatOf(2).x, seatOf(2).y], [AISLE_X, seatOf(2).y], [AISLE_X, 392], [SPOTS.gateIn.x, 392], [SPOTS.gateOut.x, 392], [SPOTS.labels.x, 392],
   [RIGHT_AISLE_X, 392], [RIGHT_AISLE_X, 468], [1085, 468], [SPOTS.main.x, SPOTS.main.y]].forEach(([x, y], i) => (i ? ctx.lineTo(x, y) : ctx.moveTo(x, y)));
  ctx.stroke(); ctx.setLineDash([]);
  ctx.restore();
}
function roomAt(x, y) {
  if (Math.abs(x - WALL_X) < 40 && y > DOOR.y1 - 30 && y < DOOR.y2 + 30) return "gate";
  return TOUR.find((id) => id !== "gate" && ROOMS[id] && x >= ROOMS[id].x && x <= ROOMS[id].x + ROOMS[id].w && y >= ROOMS[id].y && y <= ROOMS[id].y + ROOMS[id].h) || null;
}
function showExplain(id) {
  selected = id;
  const [title, body] = EXPLAIN[id] || ["How Airlock works", "Pick a room, or follow the tour."];
  document.getElementById("explain-title").textContent = title;
  document.getElementById("explain-body").textContent = body;
  const i = TOUR.indexOf(id);
  document.getElementById("tour-step").textContent = i >= 0 ? `${i + 1} of ${TOUR.length}` : "";
}

// ── Frame ────────────────────────────────────────────────────────────────────

let last = performance.now();
function frame(t) {
  now = t;
  const dt = Math.min(0.05, (t - last) / 1000); last = t;
  for (const a of agents.values()) { a.update(dt); if (a.walking) a.phase += dt * 14; }
  for (const p of parcels.values()) p.update(dt);
  // Carried parcels ride along with their agent.
  // Carried cards ride with their agent, stacked.
  const stacks = new Map();
  for (const p of parcels.values()) {
    if (!p.carriedBy) continue;
    const k = stacks.get(p.carriedBy) || 0; stacks.set(p.carriedBy, k + 1);
    p.x = p.carriedBy.x + 14; p.y = p.carriedBy.y - 18 - k * 5;
  }

  drawPaper();
  ["desks", "lab", "labels", "review", "studio", "queue", "main", "doctor"].forEach((id, i) => drawRoom(id, 10 + i * 17));
  drawWall(); drawDesks(); drawLab(); drawReview(); drawBelt(); drawMain(); drawStudio();
  Object.entries(STAFF).forEach(([id, s]) => {
    const f = flashes[id]; const busy = f && now < f.until;
    drawPerson(s.x, s.y + (busy && !reducedMotion ? Math.sin(now / 90) * 1.5 : 0), s.color, { hat: s.hat });
    text(s.name, s.x, s.y + 16, { size: 10, weight: 700, color: T.muted, align: "center" });
  });
  for (const p of [...parcels.values()]) drawParcel(p);
  const sorted = [...agents.values()].sort((a, b) => a.y - b.y);
  for (const a of sorted) {
    drawPerson(a.x, a.y, a.color, { walking: a.walking, phase: a.phase, slump: a.mode === "slump", outline: a.rogue ? T.bad : T.ink });
    if (a.mode === "thinking" && !a.walking && !reducedMotion) {
      for (let k = 0; k < 3; k += 1) {
        ctx.globalAlpha = 0.35 + 0.65 * ((Math.floor(now / 300) + k) % 3 === 0 ? 1 : 0.3);
        ctx.beginPath(); ctx.arc(a.x + 12 + k * 6, a.y - 48 - k * 5, 2.2 + k, 0, Math.PI * 2); ctx.fillStyle = T.ink; ctx.fill();
      }
      ctx.globalAlpha = 1;
    }
  }
  for (const a of sorted) if (a.bubble && now < a.bubble.until) drawBubble(a.x, a.y, a.bubble.text, a.bubble.color);
  if (mode === "explain") drawExplain();
  requestAnimationFrame(frame);
}

function resize() {
  const box = canvas.parentElement.getBoundingClientRect();
  const cssW = Math.max(box.width, 880);
  const dpr = window.devicePixelRatio || 1;
  canvas.style.width = `${cssW}px`; canvas.style.height = `${(cssW * H) / W}px`;
  canvas.width = Math.round(cssW * dpr); canvas.height = Math.round(((cssW * H) / W) * dpr);
  ctx.setTransform((cssW * dpr) / W, 0, 0, (cssW * dpr) / W, 0, 0);
}
new ResizeObserver(resize).observe(canvas.parentElement);
resize();

// ── Data ─────────────────────────────────────────────────────────────────────

const status = (s) => { document.getElementById("floor-status").textContent = s; };
let cursor = 0;
let liveTimer = null;

function reset() {
  agents.clear(); parcels.clear(); counters.merged = 0; screen = null;
  Object.keys(flashes).forEach((k) => delete flashes[k]);
  document.getElementById("floor-log").replaceChildren();
}

async function loadState() {
  const s = await (await fetch(urls.state)).json();
  // The log starts with what happened just before: described, not animated.
  const recent = await (await fetch(withParams(urls.events, { after: Math.max(0, s.cursor - LOG_MAX) }))).json().catch(() => ({ events: [] }));
  recent.events.forEach((ev) => log(ev, describe(ev)));
  sandboxOn = !!s.sandbox;
  if (s.render) loadScreen(s.render.sha, s.render.board);
  cursor = s.cursor;
  counters.merged = s.counts.merged;
  s.agents.forEach((a) => { const ag = agentFor(a.name); if (a.status === "gave_up") ag.mode = "slump"; });
  const perStation = {};
  s.changes.filter((c) => !["main", "returned"].includes(c.station)).forEach((c) => {
    const i = (perStation[c.station] = (perStation[c.station] ?? -1) + 1);
    const at = c.station === "review" ? reviewSlot(i) : c.station === "queue" ? beltSlot(i) : c.station === "doctor" ? doctorSlot(i) : SPOTS.labels;
    const p = newParcel({ change: c.id, task: c.task, actor: c.agent }, at);
    p.station = c.station;
    if (c.station === "review") p.stamp = { text: "?", color: T.warn };
    if (c.station === "doctor") p.stamp = { text: c.diagnosis ? "Dx" : "✕", color: c.diagnosis ? T.accent : T.bad };
  });
  status(`${s.agents.length} agents · ${counters.merged} merged · ${s.counts.rejected_at_gate} pushes refused at the gate`);
}

async function poll() {
  try {
    const res = await fetch(withParams(urls.events, { after: cursor }));
    const body = await res.json();
    body.events.forEach(apply);
    cursor = body.cursor;
  } catch (_) { /* keep the scene; try again */ }
}

function startLive() {
  stopReplay(); reset();
  loadState();
  clearInterval(liveTimer);
  liveTimer = setInterval(poll, 1500);
}

// Replay: real events of a past swarm, with long waits (Rust builds) squeezed.
let replay = null;
async function startReplay() {
  clearInterval(liveTimer);
  const sel = document.getElementById("floor-session");
  const opt = sel.selectedOptions[0];
  if (!opt) { status("No swarm to replay yet."); return; }
  const { from, to } = JSON.parse(opt.value);
  const events = [];
  let after = 0;
  for (let guard = 0; guard < 20; guard += 1) {
    const b = await (await fetch(withParams(urls.events, { from, to, after }))).json();
    events.push(...b.events);
    if (b.events.length < 500) break;
    after = b.cursor;
  }
  if (!events.length) { status("Nothing to replay in that window."); return; }
  reset(); sandboxOn = false;
  const speed = Number(document.getElementById("floor-speed").value);
  // Real gaps, sped up; long waits (a Rust build) capped at 8 s of real time;
  // and never less than 0.9 s between events, so every one can be seen.
  let v = 0;
  const times = events.map((e, i) => {
    if (i) v += Math.max(900, Math.min(Date.parse(e.at) - Date.parse(events[i - 1].at), 8000 * speed) / speed);
    return v;
  });
  replay = { events, times, start: performance.now(), i: 0 };
  const tick = () => {
    if (!replay) return;
    const t = performance.now() - replay.start;
    while (replay.i < replay.events.length && replay.times[replay.i] <= t) apply(replay.events[replay.i++]);
    status(`Replaying ${replay.i}/${replay.events.length} events at ${speed}×`);
    if (replay.i < replay.events.length) replay.timer = setTimeout(tick, 60);
    else status(`Replay finished: ${replay.events.length} events`);
  };
  tick();
}
function stopReplay() { if (replay?.timer) clearTimeout(replay.timer); replay = null; }

async function loadSessions() {
  const list = await (await fetch(urls.sessions)).json().catch(() => []);
  const sel = document.getElementById("floor-session");
  sel.replaceChildren(...list.map((s) => {
    const o = document.createElement("option");
    o.value = JSON.stringify(s);
    o.textContent = `${new Date(s.from).toLocaleString([], { month: "short", day: "numeric", hour: "2-digit", minute: "2-digit" })} · ${s.events} events · ${s.agents.join(", ") || "no agents"}`;
    return o;
  }));
}

// ── Controls ─────────────────────────────────────────────────────────────────

function setMode(m) {
  mode = m;
  document.querySelectorAll(".modes .mode").forEach((b) => {
    const on = b.dataset.mode === m; b.classList.toggle("active", on); b.setAttribute("aria-pressed", on);
  });
  document.querySelector(".replay-controls").hidden = m !== "replay";
  document.getElementById("floor-explain").hidden = m !== "explain";
  if (m === "live") startLive();
  if (m === "replay") loadSessions();
  if (m === "explain") showExplain(selected || TOUR[0]);
}
document.querySelectorAll(".modes .mode").forEach((b) => b.addEventListener("click", () => setMode(b.dataset.mode)));
document.getElementById("floor-play").addEventListener("click", () => { stopReplay(); startReplay(); });
document.getElementById("tour-next").addEventListener("click", () => showExplain(TOUR[(TOUR.indexOf(selected) + 1) % TOUR.length]));
document.getElementById("tour-prev").addEventListener("click", () => showExplain(TOUR[(TOUR.indexOf(selected) - 1 + TOUR.length) % TOUR.length]));
canvas.addEventListener("click", (e) => {
  const r = canvas.getBoundingClientRect();
  const id = roomAt(((e.clientX - r.left) * W) / r.width, ((e.clientY - r.top) * H) / r.height);
  if (!id) return;
  if (mode !== "explain") setMode("explain");
  showExplain(id);
});

// For tests and demos: drive the floor with events directly.
window.airlockFloor = { apply, reset, agents, parcels, setMode };

startLive();
requestAnimationFrame(frame);
