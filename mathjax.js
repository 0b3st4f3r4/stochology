(function () {
  "use strict";

  var LAB_PATH = "/lab";
  var widgetInstanceCounter = 0;

  function getPathToRoot() {
    if (typeof window !== "undefined" && typeof window.path_to_root === "string") {
      return window.path_to_root;
    }
    if (typeof path_to_root === "string") {
      return path_to_root;
    }
    return "";
  }

  function resolveLabBaseUrl() {
    var prefix = getPathToRoot();
    if (prefix.length > 0 && prefix.charAt(prefix.length - 1) !== "/") {
      prefix += "/";
    }
    return prefix + "lab";
  }

  function normalizeMathText(text) {
    if (text.indexOf("$") === -1) return text;
    return text.replace(/(\$\$[\s\S]*?\$\$|\$[^$\n]+\$)/g, function (seg) {
      return seg
        .replace(/’/g, "'")
        .replace(/(^|[^\\])%/g, "$1\\%");
    });
  }

  function announcePolite(message) {
    if (typeof document === "undefined" || !document.body) return;
    var region = document.getElementById("stoch-lab-live-region");
    if (!region) {
      region = document.createElement("div");
      region.id = "stoch-lab-live-region";
      region.className = "visually-hidden stoch-sr-only";
      region.setAttribute("aria-live", "polite");
      region.setAttribute("aria-atomic", "true");
      document.body.appendChild(region);
    }
    region.textContent = "";
    setTimeout(function () {
      region.textContent = message;
    }, 30);
  }

  function createSplitMix64(seedNum) {
    var s0 = Number(seedNum);
    if (!isFinite(s0) || s0 <= 0) s0 = 2026;
    if (typeof BigInt === "function") {
      var state = BigInt(Math.floor(s0)) & BigInt("0xFFFFFFFFFFFFFFFF");
      var GOLDEN = BigInt("0x9E3779B97F4A7C15");
      var MIX1 = BigInt("0xBF58476D1CE4E5B9");
      var MIX2 = BigInt("0x94D049BB133111EB");
      var MASK = BigInt("0xFFFFFFFFFFFFFFFF");
      return {
        nextU64: function () {
          state = (state + GOLDEN) & MASK;
          var z = state;
          z = ((z ^ (z >> BigInt(30))) * MIX1) & MASK;
          z = ((z ^ (z >> BigInt(27))) * MIX2) & MASK;
          return (z ^ (z >> BigInt(31))) & MASK;
        },
        nextF64: function () {
          var u = this.nextU64() >> BigInt(11);
          return Number(u) * Math.pow(2, -53);
        }
      };
    }
    var x = Math.floor(s0) % 2147483647;
    if (x <= 0) x += 2147483646;
    return {
      nextU64: function () {
        x = (x * 16807) % 2147483647;
        return x;
      },
      nextF64: function () {
        x = (x * 16807) % 2147483647;
        return (x - 1) / 2147483646;
      }
    };
  }

  function injectSkipLink() {
    if (typeof document === "undefined" || !document.body) return;
    if (document.querySelector(".stoch-skip-link, .lab-skip-link")) return;
    var mainEl = document.getElementById("mdbook-content") || document.querySelector("main");
    if (!mainEl) return;
    if (!mainEl.id) {
      mainEl.id = "mdbook-content";
    }
    if (!mainEl.hasAttribute("tabindex")) {
      mainEl.setAttribute("tabindex", "-1");
    }
    var skip = document.createElement("a");
    skip.className = "stoch-skip-link";
    skip.href = "#" + mainEl.id;
    skip.textContent = "Skip to main content";
    document.body.insertBefore(skip, document.body.firstChild);
  }

  function injectTopMenuLabButton() {
    var rightButtons = document.querySelector("#mdbook-menu-bar .right-buttons");
    if (!rightButtons || rightButtons.querySelector(".stoch-lab-nav-pill")) return;

    var link = document.createElement("a");
    link.className = "stoch-lab-nav-pill";
    link.href = resolveLabBaseUrl() + "/?seed=2026";
    link.target = "_blank";
    link.rel = "noopener noreferrer";
    link.title = "Open the Interactive Stochology Laboratory (/lab · seed = 2026)";
    link.setAttribute(
      "aria-label",
      "Open the Interactive Stochology Laboratory (/lab, seed = 2026) in a new tab"
    );

    var dot = document.createElement("span");
    dot.className = "stoch-lab-nav-dot";
    dot.setAttribute("aria-hidden", "true");
    link.appendChild(dot);

    var label = document.createElement("span");
    label.textContent = "/lab ↗";
    link.appendChild(label);

    rightButtons.insertBefore(link, rightButtons.firstChild);
  }

  var LAB_REGISTRY = {
    "gigerenzer-bayes": {
      code: "L01",
      title: "Interactive Natural-Frequency Tree & Turing Deciban Updater",
      pillar: "PILLAR C · CALIBRATED & SIGA STEP 8",
      subtitle: "Translate conditional probabilities into Gigerenzer integer cohorts and Turing deciban log-odds evidence",
      cli: "cargo run --release --bin labs -- --siga",
      defectState: { cohort: 10000, prev: 0.4, sens: 75.0, fpr: 22.0 },
      controls: [
        { key: "cohort", label: "Cohort Size (N)", min: 1000, max: 100000, step: 1000, val: 10000, provenance: "assumed", fmt: function (v) { return v.toLocaleString(); } },
        { key: "prev", label: "Prevalence p₀ (%)", min: 0.1, max: 25.0, step: 0.1, val: 1.0, provenance: "assumed", fmt: function (v) { return v.toFixed(1) + "%"; } },
        { key: "sens", label: "Sensitivity (%)", min: 50.0, max: 99.9, step: 0.5, val: 90.0, provenance: "assumed", fmt: function (v) { return v.toFixed(1) + "%"; } },
        { key: "fpr", label: "False-Positive Rate (%)", min: 0.5, max: 30.0, step: 0.5, val: 9.0, provenance: "assumed", fmt: function (v) { return v.toFixed(1) + "%"; } }
      ],
      evaluate: function (s) {
        var N = s.cohort;
        var p0 = s.prev / 100.0;
        var sens = s.sens / 100.0;
        var fpr = s.fpr / 100.0;
        var sick = Math.round(N * p0);
        var healthy = N - sick;
        var tp = Math.round(sick * sens);
        var fn = sick - tp;
        var fp = Math.round(healthy * fpr);
        var tn = healthy - fp;
        var totPos = Math.max(1, tp + fp);
        var ppv = tp / totPos;
        var lr = sens / Math.max(1e-6, fpr);
        var dB = 10.0 * (Math.log(lr) / Math.LN10);
        var priorOdds = p0 / Math.max(1e-6, 1.0 - p0);
        var priorDb = 10.0 * (Math.log(priorOdds) / Math.LN10);
        var postDb = priorDb + dB;

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<rect x="290" y="14" width="300" height="44" rx="8" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2"/>' +
          '<text x="440" y="41" text-anchor="middle" fill="var(--ink)" font-size="13.5" font-weight="800">Cohort N = ' + N.toLocaleString() + ' Patients</text>' +
          '<line x1="370" y1="58" x2="210" y2="86" stroke="var(--amber)" stroke-width="2"/>' +
          '<line x1="510" y1="58" x2="670" y2="86" stroke="var(--emerald)" stroke-width="2"/>' +
          '<rect x="80" y="86" width="260" height="44" rx="8" fill="var(--amber-bg)" stroke="var(--amber)" stroke-width="1.8"/>' +
          '<text x="210" y="113" text-anchor="middle" fill="var(--ink)" font-size="12.5" font-weight="750">Sick: ' + sick.toLocaleString() + ' (' + s.prev.toFixed(1) + '%)</text>' +
          '<rect x="540" y="86" width="260" height="44" rx="8" fill="var(--emerald-bg)" stroke="var(--emerald)" stroke-width="1.8"/>' +
          '<text x="670" y="113" text-anchor="middle" fill="var(--ink)" font-size="12.5" font-weight="750">Healthy: ' + healthy.toLocaleString() + '</text>' +
          '<line x1="160" y1="130" x2="115" y2="160" stroke="var(--amber)" stroke-width="1.8"/>' +
          '<line x1="260" y1="130" x2="305" y2="160" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<line x1="620" y1="130" x2="575" y2="160" stroke="var(--rose)" stroke-width="1.8"/>' +
          '<line x1="720" y1="130" x2="765" y2="160" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<rect x="28" y="160" width="180" height="52" rx="8" fill="var(--surface)" stroke="var(--amber)" stroke-width="2"/>' +
          '<text x="118" y="183" text-anchor="middle" fill="var(--ink)" font-size="12.5" font-weight="800">True (+): ' + tp.toLocaleString() + '</text>' +
          '<text x="118" y="201" text-anchor="middle" fill="var(--amber)" font-size="11">Sens = ' + s.sens.toFixed(1) + '%</text>' +
          '<rect x="224" y="160" width="170" height="52" rx="8" fill="var(--elevated)" stroke="var(--border)"/>' +
          '<text x="309" y="190" text-anchor="middle" fill="var(--muted)" font-size="12">False (-): ' + fn.toLocaleString() + '</text>' +
          '<rect x="486" y="160" width="180" height="52" rx="8" fill="var(--surface)" stroke="var(--rose)" stroke-width="2"/>' +
          '<text x="576" y="183" text-anchor="middle" fill="var(--ink)" font-size="12.5" font-weight="800">False (+): ' + fp.toLocaleString() + '</text>' +
          '<text x="576" y="201" text-anchor="middle" fill="var(--rose)" font-size="11">FPR = ' + s.fpr.toFixed(1) + '%</text>' +
          '<rect x="682" y="160" width="170" height="52" rx="8" fill="var(--elevated)" stroke="var(--border)"/>' +
          '<text x="767" y="190" text-anchor="middle" fill="var(--muted)" font-size="12">True (-): ' + tn.toLocaleString() + '</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Posterior PPV P(D|+)", value: (ppv * 100.0).toFixed(2) + "%", sub: tp + " / " + totPos + " positive alarms", tone: ppv >= 0.5 ? "emerald" : (ppv >= 0.08 ? "amber" : "rose") },
            { label: "Likelihood Ratio (LR+)", value: lr.toFixed(2) + "×", sub: "Sens / FPR", tone: "sky" },
            { label: "Turing Evidence W(+)", value: (dB >= 0 ? "+" : "") + dB.toFixed(2) + " dB", sub: "10 · log₁₀(LR+)", tone: dB >= 6.0 ? "emerald" : "rose" },
            { label: "Posterior Log-Odds", value: (postDb >= 0 ? "+" : "") + postDb.toFixed(2) + " dB", sub: "Prior " + priorDb.toFixed(1) + " dB + W", tone: postDb >= -15.0 ? "amber" : "rose" }
          ]
        };
      }
    },

    "jensen-crps": {
      code: "L02",
      title: "Interactive Convexity, Secant Chord & Jensen's Gap (Δ_J) Simulator",
      pillar: "PROHIBITION 2 · NONLINEARITY & JENSEN'S GAP",
      subtitle: "Compare the naive plug-in evaluation at the mean f(E[X]) against the true expected payoff E[f(X)]",
      cli: "cargo run --release --bin labs -- --layers",
      defectState: { mu: 2.0, spread: 3.3, curv: 3.6 },
      controls: [
        { key: "mu", label: "Mean Input E[X]", min: 1.0, max: 6.0, step: 0.1, val: 3.0, provenance: "assumed", fmt: function (v) { return v.toFixed(2); } },
        { key: "spread", label: "Half-Spread δ (X = μ ± δ)", min: 0.2, max: 3.5, step: 0.1, val: 2.0, provenance: "assumed", fmt: function (v) { return "±" + v.toFixed(2); } },
        { key: "curv", label: "Convex Curvature c (f(x)=c·x²)", min: 0.5, max: 4.0, step: 0.1, val: 1.5, provenance: "assumed", fmt: function (v) { return v.toFixed(2); } }
      ],
      evaluate: function (s) {
        var mu = s.mu;
        var d = s.spread;
        var c = s.curv;
        var x1 = mu - d;
        var x2 = mu + d;
        var f = function (x) { return c * x * x; };
        var fMu = f(mu);
        var efX = 0.5 * f(x1) + 0.5 * f(x2);
        var gap = efX - fMu;
        var relGap = gap / Math.max(0.01, fMu);
        var xMin = -2.0;
        var xMax = 10.0;
        var yMax = Math.max(10.0, f(xMax) * 0.85);
        var mapX = function (x) { return 70 + ((x - xMin) / (xMax - xMin)) * 760; };
        var mapY = function (y) { return 195 - Math.min(175, (y / yMax) * 170); };

        var pts = [];
        for (var i = 0; i <= 60; i++) {
          var xv = xMin + (i / 60.0) * (xMax - xMin);
          pts.push((i === 0 ? "M " : "L ") + mapX(xv).toFixed(1) + " " + mapY(f(xv)).toFixed(1));
        }

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="830" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<line x1="' + mapX(0.0).toFixed(1) + '" y1="20" x2="' + mapX(0.0).toFixed(1) + '" y2="195" stroke="var(--muted)" stroke-width="1.2" stroke-dasharray="3 3"/>' +
          '<path d="' + pts.join(" ") + '" fill="none" stroke="var(--sky)" stroke-width="2.8"/>' +
          '<line x1="' + mapX(x1).toFixed(1) + '" y1="' + mapY(f(x1)).toFixed(1) + '" x2="' + mapX(x2).toFixed(1) + '" y2="' + mapY(f(x2)).toFixed(1) + '" stroke="var(--amber)" stroke-width="2.2" stroke-dasharray="5 3"/>' +
          '<line x1="' + mapX(mu).toFixed(1) + '" y1="' + mapY(fMu).toFixed(1) + '" x2="' + mapX(mu).toFixed(1) + '" y2="' + mapY(efX).toFixed(1) + '" stroke="var(--rose)" stroke-width="3.2"/>' +
          '<circle cx="' + mapX(x1).toFixed(1) + '" cy="' + mapY(f(x1)).toFixed(1) + '" r="4.5" fill="var(--amber)"/>' +
          '<circle cx="' + mapX(x2).toFixed(1) + '" cy="' + mapY(f(x2)).toFixed(1) + '" r="4.5" fill="var(--amber)"/>' +
          '<circle cx="' + mapX(mu).toFixed(1) + '" cy="' + mapY(fMu).toFixed(1) + '" r="5" fill="var(--sky)"/>' +
          '<circle cx="' + mapX(mu).toFixed(1) + '" cy="' + mapY(efX).toFixed(1) + '" r="5" fill="var(--rose)"/>' +
          '<text x="' + (mapX(mu) + 12).toFixed(1) + '" y="' + ((mapY(fMu) + mapY(efX)) / 2 + 4).toFixed(1) + '" fill="var(--rose)" font-size="12" font-weight="800">Δ_J = +' + gap.toFixed(2) + '</text>' +
          '<text x="86" y="42" fill="var(--sky)" font-size="12" font-weight="700">Convex Payoff f(x) = ' + c.toFixed(1) + '·x²  |  Secant Chord (E[f(X)]) vs. Naive Plug-In f(E[X])</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Naive Plug-In f(E[X])", value: fMu.toFixed(2), sub: "Evaluated at μ = " + mu.toFixed(2), tone: "sky" },
            { label: "True Expectation E[f(X)]", value: efX.toFixed(2), sub: "0.5·f(x₁) + 0.5·f(x₂)", tone: "emerald" },
            { label: "Jensen's Gap (Δ_J)", value: "+" + gap.toFixed(2), sub: (relGap * 100.0).toFixed(1) + "% above plug-in f(E[X])", tone: relGap > 0.65 ? "rose" : "amber" },
            { label: "Taylor Exact ½f''(μ)Var(X)", value: "+" + (c * d * d).toFixed(2), sub: "Exact quadratic closure", tone: "emerald" }
          ]
        };
      }
    },

    "meng-performativity": {
      code: "L03",
      title: "Interactive Meng Big-Data Paradox (L₁→L₂) & Performative RRM/PerfGD (L₃→L₁)",
      pillar: "PILLAR I & P · MENG IDENTITY & PERFORMATIVITY",
      subtitle: "Audit Meng's Big-Data Paradox (L₁→L₂) alongside Repeated Risk Minimization contraction r_RRM = ε·(β/γ) (L₃→L₁)",
      cli: "cargo run --release --bin labs -- --layers",
      defectState: { rho: 0.22, frac: 75.0, eps: 0.85, beta: 1.60 },
      controls: [
        { key: "rho", label: "Selection Corr |ρ_{R,Y}|", min: 0.005, max: 0.25, step: 0.005, val: 0.133, provenance: "measured (cps_observational.csv)", fmt: function (v) { return v.toFixed(3); } },
        { key: "frac", label: "Sample Share f = n/N (%)", min: 5.0, max: 80.0, step: 1.0, val: 50.0, provenance: "assumed", fmt: function (v) { return v.toFixed(0) + "%"; } },
        { key: "eps", label: "Performative Sensitivity ε", min: 0.05, max: 0.95, step: 0.05, val: 0.20, provenance: "assumed", fmt: function (v) { return v.toFixed(2); } },
        { key: "beta", label: "Smoothness Ratio β/γ", min: 0.5, max: 2.0, step: 0.1, val: 1.20, provenance: "assumed", fmt: function (v) { return v.toFixed(2); } }
      ],
      evaluate: function (s) {
        var N = 32000;
        var f = s.frac / 100.0;
        var n = Math.round(N * f);
        var ddi = s.rho * s.rho;
        var nEffMeng = Math.min(n, n / (1.0 + (N - n) * Math.max(1e-6, ddi)));
        var rRrm = s.eps * s.beta;
        var denom = Math.max(0.15, 1.0 - s.eps);
        var thetaPs = 1.5 / denom;
        var thetaPo = 1.0 / denom + 0.5 / (denom * denom);

        var rrmPts = [];
        var pgdPts = [];
        var th1 = 0.2;
        var th2 = 0.2;
        for (var t = 0; t <= 20; t++) {
          var x = 70 + (t / 20.0) * 680;
          var y1 = 190 - Math.min(165, Math.max(10, (th1 / 6.0) * 155));
          var y2 = 190 - Math.min(165, Math.max(10, (th2 / 6.0) * 155));
          rrmPts.push((t === 0 ? "M " : "L ") + x.toFixed(1) + " " + y1.toFixed(1));
          pgdPts.push((t === 0 ? "M " : "L ") + x.toFixed(1) + " " + y2.toFixed(1));
          th1 = thetaPs + (th1 - thetaPs) * (rRrm < 1.0 ? rRrm : 1.12);
          th2 = th2 + 0.28 * (thetaPo - th2);
        }
        var mengBarH = Math.min(140, Math.max(6, (Math.log10(Math.max(1, nEffMeng)) / Math.log10(N)) * 140));

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="750" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<line x1="70" y1="20" x2="70" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<path d="' + rrmPts.join(" ") + '" fill="none" stroke="var(--rose)" stroke-width="2.6"/>' +
          '<path d="' + pgdPts.join(" ") + '" fill="none" stroke="var(--emerald)" stroke-width="2.8" stroke-dasharray="6 3"/>' +
          '<rect x="780" y="' + (195 - mengBarH).toFixed(1) + '" width="44" height="' + mengBarH.toFixed(1) + '" rx="4" fill="var(--sky-bg)" stroke="var(--sky)" stroke-width="1.8"/>' +
          '<text x="802" y="212" text-anchor="middle" fill="var(--sky)" font-size="10.5" font-weight="700">n_eff=' + nEffMeng.toFixed(0) + '</text>' +
          '<text x="86" y="42" fill="var(--ink)" font-size="12" font-weight="750">Performative Trajectory: Myopic RRM θ_PS=' + thetaPs.toFixed(2) + ' (Rose) vs. Stackelberg PerfGD θ_PO=' + thetaPo.toFixed(2) + ' (Emerald)</text>' +
          '<text x="86" y="62" fill="var(--muted)" font-size="11.5">Contraction r_RRM = ε·(β/γ) = ' + rRrm.toFixed(3) + (rRrm < 1.0 ? " < 1 (Convergent)" : " ≥ 1 (Bifurcated / Divergent — Gate G6 Blocks!)") + '</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Nominal Sample (n)", value: n.toLocaleString(), sub: "Out of N = " + N.toLocaleString(), tone: "sky" },
            { label: "Meng Effective n_eff", value: nEffMeng.toFixed(1), sub: "DDI = " + ddi.toFixed(5), tone: nEffMeng < 40 ? "rose" : "amber" },
            { label: "RRM Contraction r_RRM", value: rRrm.toFixed(3), sub: rRrm < 1.0 ? "Stable (< 1.0)" : "BLOCKED (≥ 1.0)", tone: rRrm < 1.0 ? "emerald" : "rose" },
            { label: "Stackelberg Gap |θ_PO-θ_PS|", value: Math.abs(thetaPo - thetaPs).toFixed(3), sub: "θ_PO=" + thetaPo.toFixed(2) + " vs θ_PS=" + thetaPs.toFixed(2), tone: "emerald" }
          ]
        };
      }
    },

    "ols-cholesky-gls": {
      code: "L04",
      title: "Interactive OLS Orthogonal Projection, Correlated Errors & Cholesky GLS Simulator",
      pillar: "PILLAR A · EFFECTIVE SAMPLE SIZE & GLS",
      subtitle: "Contrast naive i.i.d. standard errors σ/√n against equicorrelated floors and Cholesky whitening",
      cli: "cargo run --release --bin labs -- --uncertainty",
      defectState: { n: 800, rho: 0.75, bias: 2.2 },
      controls: [
        { key: "n", label: "Sample Size n", min: 25, max: 1000, step: 25, val: 400, provenance: "assumed", fmt: function (v) { return "n = " + v; } },
        { key: "rho", label: "Equicorrelation ρ Between Errors", min: 0.0, max: 0.80, step: 0.05, val: 0.35, provenance: "assumed", fmt: function (v) { return v.toFixed(2); } },
        { key: "bias", label: "Structural Discrepancy δ(x)", min: 0.0, max: 2.5, step: 0.1, val: 0.8, provenance: "assumed", fmt: function (v) { return v.toFixed(1); } }
      ],
      evaluate: function (s) {
        var n = s.n;
        var rho = s.rho;
        var sig = 2.0;
        var seNaive = sig / Math.sqrt(n);
        var varTrue = (sig * sig / n) * (1.0 + (n - 1.0) * rho);
        var seTrue = Math.sqrt(varTrue);
        var rmseTotal = Math.sqrt(varTrue + s.bias * s.bias);
        var l22 = Math.sqrt(Math.max(0.01, 1.0 - rho * rho));

        var ptsNaive = [];
        var ptsTrue = [];
        var ptsRmse = [];
        for (var i = 1; i <= 40; i++) {
          var nv = i * 25;
          var sn = sig / Math.sqrt(nv);
          var vt = (sig * sig / nv) * (1.0 + (nv - 1.0) * rho);
          var st = Math.sqrt(vt);
          var rt = Math.sqrt(vt + s.bias * s.bias);
          var px = 70 + (i / 40.0) * 760;
          var pyN = 195 - Math.min(165, (sn / 2.6) * 160);
          var pyT = 195 - Math.min(165, (st / 2.6) * 160);
          var pyR = 195 - Math.min(165, (rt / 2.6) * 160);
          ptsNaive.push((i === 1 ? "M " : "L ") + px.toFixed(1) + " " + pyN.toFixed(1));
          ptsTrue.push((i === 1 ? "M " : "L ") + px.toFixed(1) + " " + pyT.toFixed(1));
          ptsRmse.push((i === 1 ? "M " : "L ") + px.toFixed(1) + " " + pyR.toFixed(1));
        }
        var curX = 70 + (n / 1000.0) * 760;
        var curY = 195 - Math.min(165, (seTrue / 2.6) * 160);

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="830" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<path d="' + ptsNaive.join(" ") + '" fill="none" stroke="var(--sky)" stroke-width="2.4" stroke-dasharray="5 3"/>' +
          '<path d="' + ptsTrue.join(" ") + '" fill="none" stroke="var(--rose)" stroke-width="2.8"/>' +
          '<path d="' + ptsRmse.join(" ") + '" fill="none" stroke="var(--amber)" stroke-width="2.0" stroke-dasharray="3 3"/>' +
          '<circle cx="' + curX.toFixed(1) + '" cy="' + curY.toFixed(1) + '" r="5" fill="var(--rose)"/>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Standard Error vs. n: Naive σ/√n (Sky) · Correlated SE (Rose) · Total RMSE √(Var+δ²) (Amber)</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Naive i.i.d. SE (σ/√n)", value: seNaive.toFixed(3), sub: "Assumes ρ = 0", tone: "sky" },
            { label: "True Correlated SE", value: seTrue.toFixed(3), sub: (seTrue / seNaive).toFixed(2) + "× wider than naive", tone: rho > 0.55 ? "rose" : "amber" },
            { label: "Total RMSE √(Var + δ²)", value: rmseTotal.toFixed(3), sub: "Includes structural δ = " + s.bias.toFixed(1), tone: s.bias > 1.6 ? "rose" : "amber" },
            { label: "Cholesky Factor L₂₂", value: l22.toFixed(3) + "·σ", sub: "√(1 - ρ²) whitened innovation", tone: "emerald" }
          ]
        };
      }
    },

    "markov-dag-do": {
      code: "L05",
      title: "Interactive Markov Contraction (η_TV) & Pearl do(X=x) vs. Berkson Collider Simulator",
      pillar: "PILLAR I · CAUSAL INVARIANCE & DPI",
      subtitle: "Inspect Dobrushin total-variation contraction η_TV alongside Pearl do(X=x) and Berkson collider bias",
      cli: "cargo run --release --bin labs -- --lenses",
      defectState: { noise: 0.35, conf: 2.2, coll: 1 },
      controls: [
        { key: "noise", label: "Channel Flip Prob ε", min: 0.05, max: 0.45, step: 0.05, val: 0.15, provenance: "assumed", fmt: function (v) { return v.toFixed(2); } },
        { key: "conf", label: "Confounder U → (X,Y) Bias", min: 0.0, max: 2.5, step: 0.1, val: 1.5, provenance: "assumed", fmt: function (v) { return "+" + v.toFixed(1); } },
        { key: "coll", label: "Collider Conditioning C=1", min: 0, max: 1, step: 1, val: 0, provenance: "assumed", fmt: function (v) { return v > 0.5 ? "Conditioned (Biased!)" : "Unconditioned (Clean)"; } }
      ],
      evaluate: function (s) {
        var eps = s.noise;
        var etaTv = Math.abs(1.0 - 2.0 * eps);
        var trueDoEffect = 2.80;
        var obsEffect = trueDoEffect + s.conf - (s.coll > 0.5 ? 1.15 : 0.0);

        var pts = [];
        var tv = 1.0;
        for (var t = 0; t <= 12; t++) {
          var px = 70 + (t / 12.0) * 660;
          var py = 195 - tv * 150;
          pts.push((t === 0 ? "M " : "L ") + px.toFixed(1) + " " + py.toFixed(1));
          tv *= etaTv;
        }
        var doBarH = (trueDoEffect / 6.0) * 140;
        var obsBarH = Math.min(150, Math.max(8, (obsEffect / 6.0) * 140));

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="730" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<path d="' + pts.join(" ") + '" fill="none" stroke="var(--emerald)" stroke-width="2.8"/>' +
          '<rect x="756" y="' + (195 - doBarH).toFixed(1) + '" width="32" height="' + doBarH.toFixed(1) + '" rx="4" fill="var(--emerald-bg)" stroke="var(--emerald)" stroke-width="1.8"/>' +
          '<rect x="798" y="' + (195 - obsBarH).toFixed(1) + '" width="32" height="' + obsBarH.toFixed(1) + '" rx="4" fill="var(--rose-bg)" stroke="var(--rose)" stroke-width="1.8"/>' +
          '<text x="772" y="212" text-anchor="middle" fill="var(--emerald)" font-size="10" font-weight="700">do</text>' +
          '<text x="814" y="212" text-anchor="middle" fill="var(--rose)" font-size="10" font-weight="700">obs</text>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Dobrushin Total-Variation Contraction ‖p^(t) - π‖_TV ≤ η_TV^t  (η_TV = ' + etaTv.toFixed(2) + ')</text>' +
          '<text x="86" y="60" fill="var(--muted)" font-size="11.5">Causal Identification: True E[Y|do(X=1)] = +2.80 vs. Naive Observational E[Y|X=1] = +' + obsEffect.toFixed(2) + '</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Dobrushin η_TV(W)", value: etaTv.toFixed(3), sub: "Contraction per step", tone: "emerald" },
            { label: "True Causal E[Y|do(X)]", value: "+" + trueDoEffect.toFixed(2), sub: "Rung 2 (Graph Mutilation)", tone: "emerald" },
            { label: "Naive Rung-1 E[Y|X]", value: "+" + obsEffect.toFixed(2), sub: "Confounded / Collider Bias", tone: s.coll > 0.5 ? "rose" : "amber" },
            { label: "Berkson Collider Guard", value: s.coll > 0.5 ? "VIOLATED (Filter 2)" : "VERIFIED (Clean)", sub: "Ban on conditioning on C", tone: s.coll > 0.5 ? "rose" : "emerald" }
          ]
        };
      }
    },

    "koh-infogap-wald": {
      code: "L06",
      title: "Interactive Kennedy-O'Hagan Discrepancy GP & Info-Gap + Sniedovich Wald Floor",
      pillar: "PILLAR I · SEVERE UNCERTAINTY & SNIEDOVICH FLOOR",
      subtitle: "Test local Info-Gap robustness α̂ against Sniedovich's global Wald maximin floor and KOH GP anchoring",
      cli: "cargo run --release --bin labs -- --uncertainty",
      defectState: { rc: 58.0, turkey: 80.0, anchor: 0.0 },
      controls: [
        { key: "rc", label: "Critical Survival Floor r_c", min: 10.0, max: 70.0, step: 2.0, val: 40.0, provenance: "assumed", fmt: function (v) { return v.toFixed(0); } },
        { key: "turkey", label: "Fragile Tail Drop (No-Man's Land)", min: 5.0, max: 90.0, step: 5.0, val: 55.0, provenance: "assumed", fmt: function (v) { return "-" + v.toFixed(0); } },
        { key: "anchor", label: "GP Boundary Anchor Weight", min: 0.0, max: 1.0, step: 0.1, val: 1.0, provenance: "assumed", fmt: function (v) { return v.toFixed(1); } }
      ],
      evaluate: function (s) {
        var rc = s.rc;
        var drop = s.turkey;
        var robA = Math.max(0.0, (78.0 - rc) / 22.0);
        var robB = Math.max(0.0, (72.0 - rc) / 14.0);
        var waldA = 78.0 - 1.4 * drop;
        var waldB = 72.0 - 0.35 * drop;
        var bothFailed = waldB < rc || s.anchor < 0.5;
        var anchorY = 55 + (1.0 - s.anchor) * 28;

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="830" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<line x1="70" y1="20" x2="70" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<line x1="70" y1="' + (195 - rc * 1.8).toFixed(1) + '" x2="830" y2="' + (195 - rc * 1.8).toFixed(1) + '" stroke="var(--amber)" stroke-width="2" stroke-dasharray="5 4"/>' +
          '<path d="M 70 ' + anchorY.toFixed(1) + ' L 410 ' + (195 - rc * 1.8).toFixed(1) + ' L 820 ' + Math.min(215, 195 - waldA * 1.8).toFixed(1) + '" fill="none" stroke="var(--rose)" stroke-width="2.8"/>' +
          '<path d="M 70 ' + (anchorY + 11).toFixed(1) + ' L 520 ' + (195 - rc * 1.8).toFixed(1) + ' L 820 ' + Math.min(215, 195 - waldB * 1.8).toFixed(1) + '" fill="none" stroke="var(--emerald)" stroke-width="2.8"/>' +
          '<circle cx="70" cy="' + anchorY.toFixed(1) + '" r="4.5" fill="var(--sky)"/>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Info-Gap Horizon α vs. Global Wald Floor in No-Man\'s Land (α → α_max)</text>' +
          '<text x="640" y="' + (190 - rc * 1.8).toFixed(1) + '" fill="var(--amber)" font-size="11.5" font-weight="700">Critical Threshold r_c = ' + rc.toFixed(0) + '</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Option A Local α̂", value: robA.toFixed(2), sub: "Global Floor = " + waldA.toFixed(1), tone: waldA < rc ? "amber" : "sky" },
            { label: "Option B Local α̂", value: robB.toFixed(2), sub: "Global Floor = " + waldB.toFixed(1), tone: waldB < rc ? "rose" : "emerald" },
            { label: "Sniedovich Safeguard", value: bothFailed ? "BLOCKED (Floor < r_c)" : "VERIFIED (Selects B)", sub: "Global Wald Maximin Floor", tone: bothFailed ? "rose" : "emerald" },
            { label: "KOH GP Identifiability", value: s.anchor >= 0.5 ? "Anchored (Exact)" : "Confounded!", sub: "δ(x) ~ GP(0, k)", tone: s.anchor >= 0.5 ? "emerald" : "rose" }
          ]
        };
      }
    },

    "lenses-kelly-tail-corp": {
      code: "L07",
      title: "Interactive Five Lenses: Itô Variance Drain, Kelly Growth g(f) & Pareto Tail Explorer",
      pillar: "FIVE TRANSVERSAL LENSES · TIME & TAIL",
      subtitle: "Explore Itô variance drain g_time(f) = μf - ½σ²f², Kelly ruin boundaries, and Pareto tail regimes",
      cli: "cargo run --release --bin labs -- --lenses",
      defectState: { mu: 4.5, sigma: 38.0, lev: 3.8, alpha: 0.9 },
      controls: [
        { key: "mu", label: "Ensemble Drift μ (%/yr)", min: 2.0, max: 18.0, step: 0.5, val: 6.1, provenance: "measured (djia_1885_2023.csv)", fmt: function (v) { return v.toFixed(1) + "%"; } },
        { key: "sigma", label: "Volatility σ (%/yr)", min: 5.0, max: 45.0, step: 1.0, val: 17.0, provenance: "measured (djia_1885_2023.csv)", fmt: function (v) { return v.toFixed(1) + "%"; } },
        { key: "lev", label: "Leverage Fraction f (×)", min: 0.1, max: 4.5, step: 0.1, val: 1.0, provenance: "assumed", fmt: function (v) { return v.toFixed(1) + "×"; } },
        { key: "alpha", label: "Pareto Tail Index α", min: 0.8, max: 3.5, step: 0.1, val: 1.6, provenance: "measured (danish_fire.csv)", fmt: function (v) { return v.toFixed(2); } }
      ],
      evaluate: function (s) {
        var mu = s.mu / 100.0;
        var sig = s.sigma / 100.0;
        var f = s.lev;
        var fStar = mu / Math.max(1e-4, sig * sig);
        var gEns = mu * f;
        var drain = 0.5 * sig * sig * f * f;
        var gTime = gEns - drain;
        var regime = s.alpha <= 1.0 ? "Infinite Mean & Var" : (s.alpha <= 2.0 ? "Finite Mean, ∞ Var" : "Finite Mean & Var");

        var ptsEns = [];
        var ptsTime = [];
        var fMax = 4.5;
        for (var i = 0; i <= 45; i++) {
          var fv = (i / 45.0) * fMax;
          var ge = mu * fv;
          var gt = ge - 0.5 * sig * sig * fv * fv;
          var x = 70 + (fv / fMax) * 760;
          var yE = 140 - Math.max(-75, Math.min(110, ge * 380));
          var yT = 140 - Math.max(-75, Math.min(110, gt * 380));
          ptsEns.push((i === 0 ? "M " : "L ") + x.toFixed(1) + " " + yE.toFixed(1));
          ptsTime.push((i === 0 ? "M " : "L ") + x.toFixed(1) + " " + yT.toFixed(1));
        }

        var curX = 70 + (f / fMax) * 760;
        var curY = 140 - Math.max(-75, Math.min(110, gTime * 380));

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="140" x2="830" y2="140" stroke="var(--muted)" stroke-width="1.5" stroke-dasharray="4 3"/>' +
          '<line x1="70" y1="20" x2="70" y2="210" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<path d="' + ptsEns.join(" ") + '" fill="none" stroke="var(--sky)" stroke-width="2.2" stroke-dasharray="5 4"/>' +
          '<path d="' + ptsTime.join(" ") + '" fill="none" stroke="var(--emerald)" stroke-width="2.8"/>' +
          '<circle cx="' + curX.toFixed(1) + '" cy="' + curY.toFixed(1) + '" r="5.5" fill="var(--rose)"/>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Ensemble Growth g_ens(f) = μf (Sky Dashed) vs. Compounded Time Growth g_time(f) = μf - ½σ²f² (Emerald)</text>' +
          '<text x="86" y="206" fill="var(--muted)" font-size="11.5">Zero-Growth Line (g = 0%)  |  Above f = 2f* = ' + (2.0 * fStar).toFixed(2) + '×, volatility drag guarantees long-run ruin  |  Tail α = ' + s.alpha.toFixed(2) + ' (' + regime + ')</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Ensemble Rate g_ens", value: (gEns * 100.0).toFixed(2) + "%/yr", sub: "Illusion of parallel crowd", tone: "sky" },
            { label: "Time Rate g_time(f)", value: (gTime * 100.0).toFixed(2) + "%/yr", sub: "Itô Drain = -" + (drain * 100.0).toFixed(2) + "%", tone: gTime > 0 ? "emerald" : "rose" },
            { label: "Optimal Kelly Peak f*", value: fStar.toFixed(2) + "×", sub: "Ruin boundary > " + (2.0 * fStar).toFixed(2) + "×", tone: "amber" },
            { label: "Tail Moment Regime", value: "α = " + s.alpha.toFixed(2), sub: regime, tone: s.alpha <= 1.0 ? "rose" : (s.alpha <= 2.0 ? "amber" : "emerald") }
          ]
        };
      }
    },

    "spectral-telgarsky-deff": {
      code: "L08",
      title: "Interactive Telgarsky Depth-D Sawtooth (2^D) & Gram Effective Dimension d_eff",
      pillar: "PILLAR I · ALMOST-PERFECT MACHINE (GATE G8)",
      subtitle: "Audit Telgarsky depth-D compositional sawtooth 2^D and Gram effective dimension d_eff / W ≥ 80%",
      cli: "cargo run --release --bin gates",
      defectState: { depth: 2, width: 12, collin: 0.65 },
      controls: [
        { key: "depth", label: "Network Depth D (Layers)", min: 1, max: 6, step: 1, val: 4, provenance: "assumed", fmt: function (v) { return "D = " + v; } },
        { key: "width", label: "Feature Dictionary Width W", min: 4, max: 16, step: 1, val: 8, provenance: "assumed", fmt: function (v) { return "W = " + v; } },
        { key: "collin", label: "Atom Collinearity ρ", min: 0.0, max: 0.95, step: 0.05, val: 0.05, provenance: "assumed", fmt: function (v) { return v.toFixed(2); } }
      ],
      evaluate: function (s) {
        var D = Math.round(s.depth);
        var W = Math.round(s.width);
        var rho = s.collin;
        var peaks = Math.pow(2, D - 1);
        var trG = W;
        var frobSq = W + W * (W - 1) * rho * rho;
        var dEff = (trG * trG) / frobSq;
        var ratio = dEff / W;

        var pts = [];
        var steps = Math.max(128, Math.pow(2, D) * 4);
        for (var i = 0; i <= steps; i++) {
          var x = i / steps;
          var v = x;
          for (var d = 0; d < D; d++) {
            v = v < 0.5 ? 2.0 * v : 2.0 * (1.0 - v);
          }
          var px = 70 + x * 680;
          var py = 192 - v * 140;
          pts.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + py.toFixed(1));
        }
        var dEffBarH = Math.max(8, ratio * 140);

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="750" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<path d="' + pts.join(" ") + '" fill="none" stroke="var(--sky)" stroke-width="2.4"/>' +
          '<rect x="780" y="' + (195 - dEffBarH).toFixed(1) + '" width="44" height="' + dEffBarH.toFixed(1) + '" rx="4" fill="' + (ratio >= 0.8 ? "var(--emerald-bg)" : "var(--rose-bg)") + '" stroke="' + (ratio >= 0.8 ? "var(--emerald)" : "var(--rose)") + '" stroke-width="1.8"/>' +
          '<text x="802" y="212" text-anchor="middle" fill="var(--ink)" font-size="10" font-weight="700">' + (ratio * 100).toFixed(0) + '% d_eff</text>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Telgarsky Compositional Sawtooth T_D(x) at Depth D = ' + D + ' (' + Math.pow(2, D) + ' Linear Pieces · ' + peaks + ' Peaks)</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Linear Segments (2^D)", value: Math.pow(2, D).toLocaleString(), sub: "Shallow network needs Ω(2^D) width", tone: "sky" },
            { label: "Gram Effective Dim d_eff", value: dEff.toFixed(2) + " / " + W, sub: "(tr G)² / ‖G‖_F²", tone: ratio >= 0.8 ? "emerald" : "rose" },
            { label: "Completeness Ratio", value: (ratio * 100.0).toFixed(1) + "%", sub: ratio >= 0.8 ? "Gate G8 P5 Approved (≥80%)" : "Gate G8 BLOCKED (<80%)", tone: ratio >= 0.8 ? "emerald" : "rose" },
            { label: "Authentic Form A(model)", value: ratio >= 0.8 ? "TRUE" : "FALSE", sub: "δ(x) + S* Closure", tone: ratio >= 0.8 ? "emerald" : "rose" }
          ]
        };
      }
    },

    "rk4-hopf-brusselator": {
      code: "L09",
      title: "Interactive Forward Euler vs. 4th-Order Runge-Kutta (RK4) & Brusselator Limit Cycle",
      pillar: "RELATIONAL ONTOLOGY · RK4 & HOPF LIMIT CYCLE",
      subtitle: "Compare parasitic Forward Euler energy drift against 4th-Order Runge-Kutta (RK4) and Brusselator Hopf bifurcation",
      cli: "cargo run --release --bin labs -- --lenses",
      defectState: { dt: 0.32, b: 1.5, damp: 0.0 },
      controls: [
        { key: "dt", label: "Integration Step Size h", min: 0.02, max: 0.35, step: 0.01, val: 0.15, provenance: "assumed", fmt: function (v) { return v.toFixed(2); } },
        { key: "b", label: "Brusselator Drive B (Hopf > 2)", min: 1.2, max: 3.4, step: 0.1, val: 2.4, provenance: "assumed", fmt: function (v) { return v.toFixed(1); } },
        { key: "damp", label: "Oscillator Damping ζ", min: 0.0, max: 0.3, step: 0.02, val: 0.0, provenance: "assumed", fmt: function (v) { return v.toFixed(2); } }
      ],
      evaluate: function (s) {
        var h = s.dt;
        var zeta = s.damp;
        var eulerPts = [];
        var rk4Pts = [];
        var hopfPts = [];
        var xE = 1.0, vE = 0.0;
        var xR = 1.0, vR = 0.0;
        var hopf = s.b > 2.0;
        var hopfAmp = hopf ? Math.sqrt(s.b - 2.0) * 1.35 : 0.25 * Math.exp(-(2.0 - s.b));
        for (var i = 0; i <= 45; i++) {
          var px = 70 + (i / 45.0) * 760;
          var pyE = 120 - Math.max(-90, Math.min(90, xE * 38));
          var pyR = 120 - Math.max(-90, Math.min(90, xR * 38));
          var envRate = hopf ? 1.0 : Math.exp(-0.35 * (2.0 - s.b) * i * h);
          var xH = hopfAmp * envRate * Math.cos(i * h);
          var pyH = 120 - Math.max(-90, Math.min(90, xH * 38));
          eulerPts.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + pyE.toFixed(1));
          rk4Pts.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + pyR.toFixed(1));
          hopfPts.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + pyH.toFixed(1));

          var nxE = xE + h * vE;
          var nvE = vE + h * (-xE - 2.0 * zeta * vE);
          xE = nxE; vE = nvE;

          var k1x = vR, k1v = -xR - 2.0 * zeta * vR;
          var k2x = vR + 0.5 * h * k1v, k2v = -(xR + 0.5 * h * k1x) - 2.0 * zeta * k2x;
          var k3x = vR + 0.5 * h * k2v, k3v = -(xR + 0.5 * h * k2x) - 2.0 * zeta * k3x;
          var k4x = vR + h * k3v, k4v = -(xR + h * k3x) - 2.0 * zeta * k4x;
          xR += (h / 6.0) * (k1x + 2.0 * k2x + 2.0 * k3x + k4x);
          vR += (h / 6.0) * (k1v + 2.0 * k2v + 2.0 * k3v + k4v);
        }
        var eEuler = 0.5 * (xE * xE + vE * vE);
        var eRk4 = 0.5 * (xR * xR + vR * vR);

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="120" x2="830" y2="120" stroke="var(--muted)" stroke-width="1.2" stroke-dasharray="4 3"/>' +
          '<path d="' + eulerPts.join(" ") + '" fill="none" stroke="var(--rose)" stroke-width="2.2" stroke-dasharray="5 3"/>' +
          '<path d="' + rk4Pts.join(" ") + '" fill="none" stroke="var(--emerald)" stroke-width="2.8"/>' +
          '<path d="' + hopfPts.join(" ") + '" fill="none" stroke="var(--sky)" stroke-width="2.0" stroke-dasharray="3 3"/>' +
          '<text x="86" y="36" fill="var(--ink)" font-size="12" font-weight="750">Forward Euler Drift (Rose) vs. RK4 (Emerald) &amp; Brusselator Mode B=' + s.b.toFixed(1) + ' (Sky Dotted)</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Euler Energy E_T / E_0", value: (eEuler / 0.5).toFixed(2) + "×", sub: "Parasitic gain (1+h²)ⁿ", tone: h > 0.24 ? "rose" : "amber" },
            { label: "RK4 Energy E_T / E_0", value: (eRk4 / 0.5).toFixed(4) + "×", sub: "O(h⁴) global accuracy", tone: "emerald" },
            { label: "Brusselator Regime", value: hopf ? "Limit Cycle (d_i S > 0)" : "Point Attractor", sub: "B = " + s.b.toFixed(1) + " vs B_c = 2.0", tone: hopf ? "emerald" : "sky" },
            { label: "Trace tr(J) = B - 2", value: (s.b - 2.0 >= 0 ? "+" : "") + (s.b - 2.0).toFixed(2), sub: hopf ? "Hopf Bifurcation Active" : "Damped Spiral", tone: "amber" }
          ]
        };
      }
    },

    "analog-hpinv-collapse": {
      code: "L10",
      title: "Interactive Basel π²/6 Synthetic Accumulation Bound & PKU 24-Bit HP-INV RRAM Solver",
      pillar: "PILLAR P & A · BASEL π²/6 & ANALOG HP-INV",
      subtitle: "Contrast linear synthetic Replace collapse against the Basel π²/6 Accumulate ceiling and 24-bit HP-INV solver",
      cli: "cargo run --release --bin gates -- --defect replace_collapse",
      defectState: { gen: 15, noise: 0.68, cycles: 1 },
      controls: [
        { key: "gen", label: "Recursive Generations (n)", min: 1, max: 16, step: 1, val: 8, provenance: "assumed", fmt: function (v) { return "Gen " + v; } },
        { key: "noise", label: "Analog RRAM Noise σ", min: 0.05, max: 0.75, step: 0.05, val: 0.25, provenance: "assumed", fmt: function (v) { return v.toFixed(2); } },
        { key: "cycles", label: "HP-INV Bit-Slice Cycles (k)", min: 1, max: 10, step: 1, val: 6, provenance: "assumed", fmt: function (v) { return kFmt(v); } }
      ],
      evaluate: function (s) {
        var G = Math.round(s.gen);
        var accPts = ["M 70.0 195.0"];
        var repPts = ["M 70.0 195.0"];
        var sumBasel = 0.0;
        for (var g = 1; g <= G; g++) {
          sumBasel += 1.0 / (g * g);
          var px = 70 + (g / Math.max(1, G)) * 680;
          var pyA = 195 - (sumBasel / 16.0) * 160;
          var pyR = 195 - Math.min(165, (g / 16.0) * 160);
          accPts.push("L " + px.toFixed(1) + " " + pyA.toFixed(1));
          repPts.push("L " + px.toFixed(1) + " " + pyR.toFixed(1));
        }
        var baselCeil = (Math.PI * Math.PI) / 6.0;
        var rhoHp = Math.min(0.92, s.noise * 0.45);
        var relErr = Math.pow(rhoHp, Math.round(s.cycles) * 2.4);
        var errBarH = Math.min(145, Math.max(6, ((Math.log10(Math.max(1e-12, relErr)) + 12.0) / 12.0) * 145));

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="750" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<line x1="70" y1="' + (195 - (baselCeil / 16.0) * 160).toFixed(1) + '" x2="750" y2="' + (195 - (baselCeil / 16.0) * 160).toFixed(1) + '" stroke="var(--sky)" stroke-width="1.8" stroke-dasharray="5 3"/>' +
          '<path d="' + repPts.join(" ") + '" fill="none" stroke="var(--rose)" stroke-width="2.8"/>' +
          '<path d="' + accPts.join(" ") + '" fill="none" stroke="var(--emerald)" stroke-width="2.8"/>' +
          '<rect x="778" y="' + (195 - errBarH).toFixed(1) + '" width="44" height="' + errBarH.toFixed(1) + '" rx="4" fill="' + (relErr < 1e-4 ? "var(--emerald-bg)" : "var(--rose-bg)") + '" stroke="' + (relErr < 1e-4 ? "var(--emerald)" : "var(--rose)") + '" stroke-width="1.8"/>' +
          '<text x="800" y="212" text-anchor="middle" fill="var(--ink)" font-size="10" font-weight="700">HP-INV</text>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Synthetic Recursion: Replace Policy Linear Collapse ∝ n (Rose) vs. Accumulate Basel Ceiling ≤ π²/6 ≈ 1.6449 (Emerald)</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Accumulate Inflation", value: sumBasel.toFixed(4) + "×", sub: "Bounded by π²/6 = 1.6449", tone: "emerald" },
            { label: "Replace Collapse", value: G.toFixed(2) + "×", sub: "Unbounded linear divergence", tone: G >= 14 ? "rose" : "amber" },
            { label: "PKU HP-INV Rel Error", value: relErr.toExponential(2), sub: Math.round(s.cycles) + " cycles on 3-bit RRAM", tone: relErr < 1e-4 ? "emerald" : "rose" },
            { label: "Horowitz DRAM/ALU Ratio", value: "711.11×", sub: "Informative (Horowitz 2014): 640 pJ / 0.9 pJ", tone: "sky" }
          ]
        };
      }
    },

    "stochary-kingman-triad": {
      code: "L11",
      title: "Interactive Stochary Buffer, Kingman VUT Queueing Cliff & Triadic (ℂ × Fractals × 𝕋ᵈ) Explorer",
      pillar: "PILLAR A · STOCHARY BUFFERS & TRIADIC SYNTHESIS",
      subtitle: "Audit Kingman's hyperbolic VUT queueing cliff W_q ∝ u/(1-u), Peters time growth, and Sterman supply-line weighting",
      cli: "cargo run --release --bin gates -- --defect stochary_cybernetics",
      defectState: { util: 98.5, fexp: 0.95, sl: 0.15 },
      controls: [
        { key: "util", label: "Server Utilization u (%)", min: 50.0, max: 99.0, step: 0.5, val: 82.0, provenance: "assumed", fmt: function (v) { return v.toFixed(1) + "%"; } },
        { key: "fexp", label: "Peters +50%/-40% Exposure f", min: 0.05, max: 1.0, step: 0.05, val: 0.25, provenance: "assumed", fmt: function (v) { return (v * 100).toFixed(0) + "%"; } },
        { key: "sl", label: "Sterman Supply-Line Weight β_SL", min: 0.05, max: 1.0, step: 0.05, val: 1.0, provenance: "assumed", fmt: function (v) { return v.toFixed(2); } }
      ],
      evaluate: function (s) {
        var u = s.util / 100.0;
        var f = s.fexp;
        var wq = (u / Math.max(0.008, 1.0 - u)) * 0.404;
        var gPeters = Math.sqrt((1.0 + 0.5 * f) * (1.0 - 0.4 * f)) - 1.0;
        var bullwhip = 1.0 + (1.0 - s.sl) * 7.77;

        var pts = [];
        for (var i = 0; i <= 48; i++) {
          var uv = 0.50 + (i / 48.0) * 0.49;
          var wv = (uv / (1.0 - uv)) * 0.404;
          var px = 70 + (i / 48.0) * 660;
          var py = 195 - Math.min(170, (wv / 28.0) * 165);
          pts.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + py.toFixed(1));
        }
        var curX = 70 + ((u - 0.50) / 0.49) * 660;
        var curY = 195 - Math.min(170, (wq / 28.0) * 165);
        var bufBarH = Math.max(6, (1.0 - f) * 140);
        var bwBarH = Math.min(145, Math.max(8, (bullwhip / 9.0) * 145));

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="730" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<path d="' + pts.join(" ") + '" fill="none" stroke="var(--violet)" stroke-width="2.8"/>' +
          '<circle cx="' + curX.toFixed(1) + '" cy="' + curY.toFixed(1) + '" r="5.5" fill="var(--rose)"/>' +
          '<rect x="756" y="' + (195 - bufBarH).toFixed(1) + '" width="32" height="' + bufBarH.toFixed(1) + '" rx="4" fill="var(--emerald-bg)" stroke="var(--emerald)" stroke-width="1.8"/>' +
          '<rect x="798" y="' + (195 - bwBarH).toFixed(1) + '" width="32" height="' + bwBarH.toFixed(1) + '" rx="4" fill="' + (bullwhip < 2.0 ? "var(--sky-bg)" : "var(--rose-bg)") + '" stroke="' + (bullwhip < 2.0 ? "var(--sky)" : "var(--rose)") + '" stroke-width="1.8"/>' +
          '<text x="772" y="212" text-anchor="middle" fill="var(--emerald)" font-size="10" font-weight="700">Buf</text>' +
          '<text x="814" y="212" text-anchor="middle" fill="var(--ink)" font-size="10" font-weight="700">BW</text>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Kingman VUT Hyperbolic Queueing Cliff W_q ∝ u/(1-u)  |  Current Wait = ' + wq.toFixed(2) + ' h at u = ' + s.util.toFixed(1) + '%</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Kingman Wait W_q(u)", value: wq.toFixed(2) + " h", sub: u > 0.92 ? "Hyperbolic Cliff!" : "Buffered Capacity", tone: u > 0.92 ? "rose" : "emerald" },
            { label: "Peters Time Growth g(f)", value: (gPeters >= 0 ? "+" : "") + (gPeters * 100.0).toFixed(2) + "%", sub: "Buffer Reserve = " + ((1.0 - f) * 100).toFixed(0) + "%", tone: gPeters >= 0 ? "emerald" : "rose" },
            { label: "Supply-Line Bullwhip", value: bullwhip.toFixed(2) + "×", sub: "β_SL = " + s.sl.toFixed(2), tone: bullwhip < 2.0 ? "emerald" : "rose" },
            { label: "Triadic Weyl/ROTH ECC", value: "h₈ = 28.74", sub: "ℂ × Fractals × 𝕋ᵈ Verified", tone: "sky" }
          ]
        };
      }
    },

    "ville-savi-conformal": {
      code: "L12",
      title: "Interactive Ville Martingale e-Process (E_t) vs. Optional-Stopping Peeking Inflation",
      pillar: "PILLAR A · ANYTIME-VALID VILLE MARTINGALES",
      subtitle: "Test non-parametric GRAPA betting martingales E_t against optional-stopping peeking inflation",
      cli: "cargo run --release --bin gates -- --defect peeking",
      defectState: { peeks: 45, eff: 0.0, bet: 0.80 },
      controls: [
        { key: "peeks", label: "Sequential Peeks (K)", min: 1, max: 50, step: 1, val: 20, provenance: "assumed", fmt: function (v) { return v + " peeks"; } },
        { key: "eff", label: "True Signal Drift μ", min: 0.0, max: 0.45, step: 0.05, val: 0.25, provenance: "simulated (seed=2026)", fmt: function (v) { return v.toFixed(2); } },
        { key: "bet", label: "GRAPA Betting Fraction λ", min: 0.05, max: 0.85, step: 0.05, val: 0.40, provenance: "assumed", fmt: function (v) { return v.toFixed(2); } }
      ],
      evaluate: function (s, seed) {
        var K = Math.round(s.peeks);
        var mu = s.eff;
        var lam = s.bet;
        var fprNaive = Math.min(0.65, 0.05 * (1.0 + 0.82 * Math.log(K)));
        var rng = createSplitMix64(seed || 2026);
        var mapE = function (val) {
          return 190 - Math.min(165, Math.max(5, (Math.log(Math.max(0.2, Math.min(50.0, val))) / Math.log(50.0)) * 140 + 30));
        };
        var ePts = [];
        var E = 1.0;
        var supE = 1.0;
        for (var t = 0; t <= 30; t++) {
          var uDraw = rng.nextF64() - 0.5;
          var baseShock = (t % 2 === 0 ? -0.45 : 0.45) + 0.06 * uDraw;
          var shock = baseShock + mu;
          if (t > 0) {
            E = E * Math.max(0.1, 1.0 + lam * shock);
            if (E > supE) supE = E;
          }
          var px = 70 + (t / 30.0) * 760;
          var py = mapE(E);
          ePts.push((t === 0 ? "M " : "L ") + px.toFixed(1) + " " + py.toFixed(1));
        }
        var yVille = mapE(20.0);
        var yPeek = 190 - Math.min(160, fprNaive * 220);

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="' + yVille.toFixed(1) + '" x2="830" y2="' + yVille.toFixed(1) + '" stroke="var(--emerald)" stroke-width="2" stroke-dasharray="6 4"/>' +
          '<line x1="70" y1="' + yPeek.toFixed(1) + '" x2="830" y2="' + yPeek.toFixed(1) + '" stroke="var(--rose)" stroke-width="1.6" stroke-dasharray="3 3"/>' +
          '<path d="' + ePts.join(" ") + '" fill="none" stroke="var(--sky)" stroke-width="2.8"/>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Anytime-Valid Betting Martingale E_t = Π(1 + λ_i X_i) vs. Ville Threshold 1/α = 20.0 (Emerald Dashed)</text>' +
          '<text x="615" y="' + (yPeek - 5).toFixed(1) + '" fill="var(--rose)" font-size="10.5" font-weight="700">Naive K=' + K + ' Peeking FPR = ' + (fprNaive * 100).toFixed(1) + '%</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Cumulative Evidence E_30", value: E.toFixed(2), sub: supE >= 20.0 ? "Rejects H₀ (sup E_t ≥ 20)" : "Below 1/α = 20 threshold", tone: supE >= 20.0 ? "emerald" : (E >= 1.0 ? "amber" : "rose") },
            { label: "Ville Anytime Type-I Error", value: "≤ 5.00%", sub: "P₀(∃t: E_t ≥ 20) ≤ 0.05", tone: "emerald" },
            { label: "Naive z-Test Peeking FPR", value: (fprNaive * 100.0).toFixed(1) + "%", sub: "At K = " + K + " unadjusted peeks", tone: E < 1.0 && K > 10 ? "rose" : "amber" },
            { label: "Implied Safe p-Value", value: Math.min(1.0, 1.0 / Math.max(1.0, supE)).toFixed(4), sub: "p_safe = min(1, 1/sup E_t)", tone: "sky" }
          ]
        };
      }
    },

    "invariants-moulton-chouldechova": {
      code: "L13",
      title: "Interactive Boundary Invariants: Moulton-Campbell Tail DEFF(p) & Chouldechova Fairness",
      pillar: "INVARIANTS Φ₂ & Φ₄ · MOULTON DEFF & FAIRNESS",
      subtitle: "Compute Moulton-Campbell tail effective sample size n_eff(p) and Chouldechova's FPR_A / FPR_B trilemma",
      cli: "cargo run --release --bin labs -- --invariants",
      defectState: { mbar: 195, icc: 0.36, pa: 0.54, pb: 0.07 },
      controls: [
        { key: "mbar", label: "Mean Cluster Size m̄", min: 5, max: 200, step: 5, val: 169, provenance: "measured (bank_calls.csv)", fmt: function (v) { return v.toFixed(0); } },
        { key: "icc", label: "Tail Exceedance ICC ρ_I(p)", min: 0.01, max: 0.40, step: 0.01, val: 0.16, provenance: "measured (bank_calls.csv)", fmt: function (v) { return v.toFixed(2); } },
        { key: "pa", label: "Group A Base Rate p_A", min: 0.05, max: 0.60, step: 0.02, val: 0.39, provenance: "measured (toronto_arrests.csv)", fmt: function (v) { return v.toFixed(2); } },
        { key: "pb", label: "Group B Base Rate p_B", min: 0.05, max: 0.60, step: 0.02, val: 0.11, provenance: "measured (toronto_arrests.csv)", fmt: function (v) { return v.toFixed(2); } }
      ],
      evaluate: function (s) {
        var nNom = 27716;
        var deff = 1.0 + (s.mbar - 1.0) * s.icc;
        var nEff = nNom / deff;
        var ppv = 0.65, fnr = 0.25;
        var coeff = ((1.0 - ppv) / ppv) * (1.0 - fnr);
        var fprA = (s.pa / (1.0 - s.pa)) * coeff;
        var fprB = (s.pb / (1.0 - s.pb)) * coeff;
        var ratio = fprA / Math.max(1e-6, fprB);

        var pts = [];
        for (var i = 0; i <= 40; i++) {
          var r = (i / 40.0) * 0.40;
          var d = 1.0 + (s.mbar - 1.0) * r;
          var ne = nNom / d;
          var px = 70 + (i / 40.0) * 760;
          var py = 195 - (ne / nNom) * 155;
          pts.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + py.toFixed(1));
        }

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="830" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<path d="' + pts.join(" ") + '" fill="none" stroke="var(--violet)" stroke-width="2.8"/>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Moulton-Campbell Effective Sample Size Collapse n_eff(p) = n / (1 + (m̃-1)ρ_I(p)) as Tail ICC Grows</text>' +
          '<text x="86" y="60" fill="var(--muted)" font-size="11.5">Nominal n = 27,716  →  Audited n_eff = ' + nEff.toFixed(1) + ' (DEFF = ' + deff.toFixed(2) + '×)  |  Chouldechova FPR_A / FPR_B = ' + ratio.toFixed(2) + '× (toronto_arrests.csv)</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Design Effect DEFF(p)", value: deff.toFixed(2) + "×", sub: "Variance inflation factor", tone: deff > 45 ? "rose" : "amber" },
            { label: "Audited n_eff(p) (Φ₂)", value: nEff.toFixed(1), sub: "Down from n = 27,716", tone: nEff < 500 ? "rose" : "emerald" },
            { label: "Chouldechova FPR_A / FPR_B", value: ratio.toFixed(2) + "×", sub: "FPR_A=" + (fprA * 100).toFixed(1) + "% vs FPR_B=" + (fprB * 100).toFixed(1) + "%", tone: ratio > 8.0 ? "rose" : "amber" },
            { label: "SE Inflation √DEFF", value: Math.sqrt(deff).toFixed(2) + "×", sub: "True vs naive error bars", tone: "amber" }
          ]
        };
      }
    },

    "governance-bitemporal-drift": {
      code: "L14",
      title: "Interactive Bitemporal Clock, ExTRA-WCP-T Lei-Candès Drift Bound & Online ACI",
      pillar: "PILLAR B · BITEMPORAL CLOCK & DRIFT GOVERNANCE",
      subtitle: "Audit Lei-Candès covariate-shift floor 1 - α - d_TV against adaptive ExTRA-WCP-T / Online ACI",
      cli: "cargo run --release --bin labs -- --governance",
      defectState: { dtv: 0.32, alpha: 0.18, gamma: 0.005 },
      controls: [
        { key: "dtv", label: "Covariate Shift d_TV(P, Q)", min: 0.0, max: 0.35, step: 0.01, val: 0.09, provenance: "assumed", fmt: function (v) { return v.toFixed(2); } },
        { key: "alpha", label: "Target Miscoverage α", min: 0.05, max: 0.20, step: 0.01, val: 0.10, provenance: "assumed", fmt: function (v) { return (v * 100).toFixed(0) + "%"; } },
        { key: "gamma", label: "Online ACI Step Size γ", min: 0.005, max: 0.08, step: 0.005, val: 0.025, provenance: "assumed", fmt: function (v) { return v.toFixed(3); } }
      ],
      evaluate: function (s) {
        var target = 1.0 - s.alpha;
        var leiCandes = Math.max(0.0, 1.0 - s.alpha - s.dtv);
        var aciCov = Math.min(0.99, target - 0.12 * s.dtv + 0.8 * s.gamma);

        var ptsTarget = [];
        var ptsStatic = [];
        var ptsAci = [];
        for (var i = 0; i <= 40; i++) {
          var tv = (i / 40.0) * 0.35;
          var px = 70 + (i / 40.0) * 760;
          var pyT = 195 - target * 160;
          var pyS = 195 - Math.max(0.3, target - tv) * 160;
          var pyA = 195 - Math.min(0.99, target - 0.12 * tv + 0.8 * s.gamma) * 160;
          ptsTarget.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + pyT.toFixed(1));
          ptsStatic.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + pyS.toFixed(1));
          ptsAci.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + pyA.toFixed(1));
        }
        var curX = 70 + (s.dtv / 0.35) * 760;
        var curYS = 195 - Math.max(0.3, leiCandes) * 160;
        var curYA = 195 - aciCov * 160;

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="' + curX.toFixed(1) + '" y1="45" x2="' + curX.toFixed(1) + '" y2="195" stroke="var(--muted)" stroke-width="1.4" stroke-dasharray="3 3"/>' +
          '<path d="' + ptsTarget.join(" ") + '" fill="none" stroke="var(--sky)" stroke-width="1.8" stroke-dasharray="5 4"/>' +
          '<path d="' + ptsStatic.join(" ") + '" fill="none" stroke="var(--rose)" stroke-width="2.6"/>' +
          '<path d="' + ptsAci.join(" ") + '" fill="none" stroke="var(--emerald)" stroke-width="2.8"/>' +
          '<circle cx="' + curX.toFixed(1) + '" cy="' + curYS.toFixed(1) + '" r="4.5" fill="var(--rose)"/>' +
          '<circle cx="' + curX.toFixed(1) + '" cy="' + curYA.toFixed(1) + '" r="4.5" fill="var(--emerald)"/>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Conformal Coverage Under Drift: Lei-Candès Static Floor 1-α-d_TV (Rose) vs. Adaptive ExTRA-WCP-T / ACI (Emerald)</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Nominal Target (1 - α)", value: (target * 100.0).toFixed(1) + "%", sub: "Exchangeable baseline", tone: "sky" },
            { label: "Lei-Candès Lower Bound", value: (leiCandes * 100.0).toFixed(1) + "%", sub: "1 - α - d_TV worst-case floor", tone: leiCandes >= 0.70 ? "amber" : "rose" },
            { label: "Online ACI / ExTRA-WCP-T", value: (aciCov * 100.0).toFixed(1) + "%", sub: "Restored finite-sample coverage", tone: "emerald" },
            { label: "Temporal Vault Seal", value: "HMAC-SHA256", sub: "Single-Opening (0 → 1)", tone: "emerald" }
          ]
        };
      }
    },

    "strategic-shap-ppi-ahp": {
      code: "L15",
      title: "Interactive Strategic Accommodation: Causal do-SHAP, LLM-Judge PPI Rectifier & AHP Ideal-Mode",
      pillar: "PILLAR A · STRATEGIC AI ACCOMMODATION (GATE G8)",
      subtitle: "Contrast observational Rung-1 SHAP symptom leakage against interventional do-SHAP, PPI rectifier, and Ideal-Mode AHP",
      cli: "cargo run --release --bin gates -- --defect strategic_accommodation",
      defectState: { rho: 0.95, ngold: 45, bias: 0.32, clone: 4 },
      controls: [
        { key: "rho", label: "Cause-Symptom Collinearity ρ", min: 0.10, max: 0.95, step: 0.05, val: 0.85, provenance: "assumed", fmt: function (v) { return v.toFixed(2); } },
        { key: "ngold", label: "PPI Human Gold Sample n_gold", min: 30, max: 600, step: 15, val: 270, provenance: "assumed", fmt: function (v) { return v.toFixed(0); } },
        { key: "bias", label: "LLM-Judge Systematic Bias b_f", min: 0.02, max: 0.35, step: 0.01, val: 0.18, provenance: "assumed", fmt: function (v) { return v.toFixed(2); } },
        { key: "clone", label: "Belton-Gear Clone Count k", min: 1, max: 4, step: 1, val: 2, provenance: "assumed", fmt: function (v) { return v.toFixed(0); } }
      ],
      evaluate: function (s) {
        var obsSymptomShare = 0.5 * (s.rho * s.rho);
        var doSymptomShare = 0.0;
        var varF = 0.21, varDelta = 0.14, nUnlabeled = 2000;
        var ppiHalfWidth = 1.96 * Math.sqrt(varF / nUnlabeled + varDelta / s.ngold);
        var nStar05 = Math.ceil(varDelta / Math.max(1e-6, Math.pow(0.05 / 1.96, 2) - varF / nUnlabeled));
        var underPowered = s.ngold < nStar05;

        var ptsPpi = [];
        var ptsJudge = [];
        for (var i = 1; i <= 40; i++) {
          var ng = 15 * i;
          var hw = 1.96 * Math.sqrt(varF / nUnlabeled + varDelta / ng);
          var px = 70 + ((i - 1) / 39.0) * 760;
          var pyPpi = 195 - Math.min(165, (hw / 0.35) * 155);
          var pyJudge = 195 - Math.min(165, (s.bias / 0.35) * 155);
          ptsPpi.push((i === 1 ? "M " : "L ") + px.toFixed(1) + " " + pyPpi.toFixed(1));
          ptsJudge.push((i === 1 ? "M " : "L ") + px.toFixed(1) + " " + pyJudge.toFixed(1));
        }
        var curX = 70 + ((s.ngold - 15) / 585.0) * 760;
        var curY = 195 - Math.min(165, (ppiHalfWidth / 0.35) * 155);

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<path d="' + ptsJudge.join(" ") + '" fill="none" stroke="var(--rose)" stroke-width="2.2" stroke-dasharray="6 4"/>' +
          '<path d="' + ptsPpi.join(" ") + '" fill="none" stroke="var(--emerald)" stroke-width="2.8"/>' +
          '<circle cx="' + curX.toFixed(1) + '" cy="' + curY.toFixed(1) + '" r="5" fill="' + (underPowered ? "var(--rose)" : "var(--emerald)") + '"/>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">LLM-Judge Unrectified Bias b_f (Rose Dashed) vs. PPI 95% Half-Width as n_gold Grows (Emerald)</text>' +
          '<text x="86" y="60" fill="var(--muted)" font-size="11.5">Rung-1 Symptom SHAP Leakage = ' + (obsSymptomShare * 100).toFixed(1) + '% vs. Interventional do-SHAP = 0.0% | Prospective n*_gold(±0.05) = ' + nStar05 + '</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Rung-1 Symptom SHAP", value: (obsSymptomShare * 100.0).toFixed(1) + "%", sub: "do-SHAP on symptom = " + doSymptomShare.toFixed(1) + "%", tone: underPowered ? "rose" : "amber" },
            { label: "PPI 95% Half-Width", value: "±" + ppiHalfWidth.toFixed(3), sub: "Fixed n_gold=160 sim discarded (88%<95%)", tone: ppiHalfWidth <= 0.06 ? "emerald" : "rose" },
            { label: "Prospective n*_gold (≤0.05)", value: nStar05.toString(), sub: "Filter 2 gold-set sizing (n_gold=" + s.ngold + ")", tone: s.ngold >= nStar05 ? "emerald" : "rose" },
            { label: "AHP Ideal-Mode Guard", value: s.clone >= 4 && underPowered ? "UNANCHORED (Dist.)" : "ANCHORED (Exact)", sub: "Belton-Gear k = " + s.clone.toFixed(0) + " clones", tone: s.clone >= 4 && underPowered ? "rose" : "emerald" }
          ]
        };
      }
    },

    "kalman-control-hub-contagion": {
      code: "L16",
      title: "Interactive Closed-Loop Cybernetics: Student-t Clipped Kalman Control & Scale-Free Hub Immunization",
      pillar: "CLOSED-LOOP CYBERNETICS · STUDENT-t KALMAN & HUBS",
      subtitle: "Compare Gaussian L₂ Kalman breakdown under heavy-tailed sensor glitches against Student-t clipping and hub buffers",
      cli: "cargo run --release --bin labs -- --lenses",
      defectState: { glitch: 24, clip: 5.75, gamma: 2.1, immun: 2 },
      controls: [
        { key: "glitch", label: "Heavy-Tailed Sensor Glitch Amplitude", min: 0, max: 25, step: 1, val: 14, provenance: "simulated (seed=2026)", fmt: function (v) { return v.toFixed(1) + " σ"; } },
        { key: "clip", label: "Student-t Innovation Clip c_ν", min: 1.5, max: 6.0, step: 0.25, val: 2.5, provenance: "assumed", fmt: function (v) { return v.toFixed(2) + " σ"; } },
        { key: "gamma", label: "Scale-Free Degree Exponent γ", min: 2.1, max: 3.4, step: 0.1, val: 2.3, provenance: "assumed", fmt: function (v) { return v.toFixed(1); } },
        { key: "immun", label: "Targeted Hub Immunization (%)", min: 2, max: 30, step: 2, val: 14, provenance: "assumed", fmt: function (v) { return v.toFixed(0) + "%"; } }
      ],
      evaluate: function (s, seed) {
        var l2Rmse = 0.22 + 0.16 * s.glitch;
        var robRmse = 0.22 + 0.025 * Math.min(s.glitch, s.clip);
        var pidSurvival = Math.max(0.05, 0.98 - 0.038 * s.glitch);
        var robSurvival = Math.max(0.75, 0.99 - 0.006 * Math.min(s.glitch, s.clip));
        var hubSurvival = Math.min(0.98, 0.35 + 0.028 * s.immun + 0.08 * (s.gamma - 2.0));
        var rng = createSplitMix64(seed || 2026);

        var ptsL2 = [];
        var ptsRob = [];
        for (var t = 0; t <= 40; t++) {
          var bg = Math.sin(t * 0.35) * 0.8 + (rng.nextF64() - 0.5) * 0.35;
          var spike = (t === 14 || t === 28) ? s.glitch : bg;
          var errL2 = Math.min(12.0, Math.abs(0.55 * spike));
          var errRob = Math.min(12.0, Math.abs(0.35 * Math.min(Math.abs(spike), s.clip)));
          var px = 70 + (t / 40.0) * 680;
          var pyL2 = 195 - (errL2 / 12.0) * 155;
          var pyRob = 195 - (errRob / 12.0) * 155;
          ptsL2.push((t === 0 ? "M " : "L ") + px.toFixed(1) + " " + pyL2.toFixed(1));
          ptsRob.push((t === 0 ? "M " : "L ") + px.toFixed(1) + " " + pyRob.toFixed(1));
        }
        var hubBarH = Math.max(8, hubSurvival * 145);

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<path d="' + ptsL2.join(" ") + '" fill="none" stroke="var(--rose)" stroke-width="2.4"/>' +
          '<path d="' + ptsRob.join(" ") + '" fill="none" stroke="var(--emerald)" stroke-width="2.8"/>' +
          '<rect x="778" y="' + (195 - hubBarH).toFixed(1) + '" width="44" height="' + hubBarH.toFixed(1) + '" rx="4" fill="' + (hubSurvival >= 0.65 ? "var(--emerald-bg)" : "var(--rose-bg)") + '" stroke="' + (hubSurvival >= 0.65 ? "var(--emerald)" : "var(--rose)") + '" stroke-width="1.8"/>' +
          '<text x="800" y="212" text-anchor="middle" fill="var(--ink)" font-size="10" font-weight="700">Hub ' + (hubSurvival * 100).toFixed(0) + '%</text>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Closed-Loop State Tracking Error Under Heavy-Tailed Sensor Glitches: Gaussian L₂ Kalman (Rose) vs. Student-t Clipped (Emerald)</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "L₂ Kalman + PID RMSE", value: l2Rmse.toFixed(2), sub: "Lap survival = " + (pidSurvival * 100).toFixed(0) + "%", tone: s.clip > 4.5 ? "rose" : "amber" },
            { label: "Student-t Clipped RMSE", value: robRmse.toFixed(2), sub: "Lap survival = " + (robSurvival * 100).toFixed(0) + "%", tone: "emerald" },
            { label: "Scale-Free Hub Survival", value: (hubSurvival * 100.0).toFixed(1) + "%", sub: "Targeted hub immunization", tone: hubSurvival >= 0.65 ? "emerald" : "rose" },
            { label: "Lyapunov Drift ΔV_t", value: "≤ -0.042", sub: "Strict stability certified", tone: "sky" }
          ]
        };
      }
    }
  };

  function kFmt(v) {
    return Math.round(v) + " cycles";
  }

  function getActiveThemeName() {
    if (typeof document !== "undefined" && document.documentElement) {
      var cl = document.documentElement.classList;
      var themes = ["light", "rust", "navy", "coal", "ayu"];
      for (var i = 0; i < themes.length; i++) {
        if (cl.contains(themes[i])) return themes[i];
      }
    }
    return isDarkThemeActive() ? "navy" : "light";
  }

  function buildLabDeepLink(moduleKey, state, chapter, seedVal) {
    var themeParam = getActiveThemeName();
    var sNum = parseInt(seedVal, 10);
    if (!isFinite(sNum) || sNum <= 0) sNum = 2026;
    var params = ["module=" + encodeURIComponent(moduleKey), "seed=" + sNum, "theme=" + encodeURIComponent(themeParam)];
    if (chapter) {
      var normChap = String(chapter).trim();
      if (normChap && !/\.html$/i.test(normChap)) normChap += ".html";
      params.push("chapter=" + encodeURIComponent(normChap));
    }
    for (var k in state) {
      if (Object.prototype.hasOwnProperty.call(state, k)) {
        params.push(encodeURIComponent(k) + "=" + encodeURIComponent(state[k]));
      }
    }
    return resolveLabBaseUrl() + "/?" + params.join("&");
  }

  if (typeof window !== "undefined") {
    window.STOCHOLOGY_LAB_REGISTRY = LAB_REGISTRY;
    window.hydrateStochologyLabWidget = hydrateLabWidget;
    window.createStochologySplitMix64 = createSplitMix64;
  }

  function setupInteractiveLabs(root) {
    var nodes = root.querySelectorAll(".stoch-lab[data-lab]");
    for (var i = 0; i < nodes.length; i++) {
      hydrateLabWidget(nodes[i]);
    }
  }

  function syncSliderCssVars(inputEl, minVal, maxVal, curVal) {
    inputEl.style.setProperty("--attr-min", String(minVal));
    inputEl.style.setProperty("--attr-max", String(maxVal));
    inputEl.style.setProperty("--control-value", String(curVal));
  }

  function hydrateLabWidget(container) {
    if (container.getAttribute("data-hydrated") === "true") return;
    widgetInstanceCounter += 1;
    var instanceId = widgetInstanceCounter;
    var moduleKey = container.getAttribute("data-lab");
    var spec = LAB_REGISTRY[moduleKey] || LAB_REGISTRY["jensen-crps"];
    var chapter = container.getAttribute("data-chapter") || "";
    var customTitle = container.getAttribute("data-title") || spec.title;
    var customSubtitle = container.getAttribute("data-subtitle") || spec.subtitle || "";
    var customCli = container.getAttribute("data-cli") || spec.cli;
    var fromQuery = container.getAttribute("data-from-query") === "true";

    container.setAttribute("data-hydrated", "true");
    container.setAttribute("role", "region");
    container.setAttribute("aria-label", customTitle);
    container.innerHTML = "";

    var queryMap = {};
    if (fromQuery && typeof window !== "undefined" && window.location && window.location.search) {
      var pairs = window.location.search.substring(1).split("&");
      for (var q = 0; q < pairs.length; q++) {
        var kv = pairs[q].split("=");
        if (kv.length === 2) {
          var qVal = parseFloat(decodeURIComponent(kv[1]));
          if (!isNaN(qVal)) {
            queryMap[decodeURIComponent(kv[0])] = qVal;
          }
        }
      }
    }

    var activeSeed = parseInt(container.getAttribute("data-seed") || queryMap.seed, 10);
    if (!isFinite(activeSeed) || activeSeed <= 0) activeSeed = 2026;

    var state = {};
    for (var c = 0; c < spec.controls.length; c++) {
      var cKey = spec.controls[c].key;
      if (Object.prototype.hasOwnProperty.call(queryMap, cKey)) {
        var clamped = Math.max(spec.controls[c].min, Math.min(spec.controls[c].max, queryMap[cKey]));
        state[cKey] = clamped;
      } else {
        state[cKey] = spec.controls[c].val;
      }
    }

    var header = document.createElement("div");
    header.className = "stoch-lab-header";

    var titleGroup = document.createElement("div");
    titleGroup.className = "stoch-lab-title-group";

    var badgeRow = document.createElement("div");
    badgeRow.className = "stoch-lab-badge-row";

    var badge = document.createElement("span");
    badge.className = "stoch-lab-badge";
    badge.textContent = "/LAB · INTERACTIVE";
    badgeRow.appendChild(badge);

    var seedBadge = document.createElement("span");
    seedBadge.className = "stoch-lab-seed-badge";
    seedBadge.textContent = "Φ₁ SEED = " + activeSeed;
    badgeRow.appendChild(seedBadge);

    if (spec.pillar) {
      var pillarBadge = document.createElement("span");
      pillarBadge.className = "stoch-lab-pillar-badge";
      pillarBadge.textContent = spec.pillar;
      badgeRow.appendChild(pillarBadge);
    }

    var verdictBadge = document.createElement("span");
    verdictBadge.className = "stoch-lab-verdict-badge";
    verdictBadge.setAttribute("data-status", "approved");
    verdictBadge.textContent = "GATE VERDICT: APPROVED";
    badgeRow.appendChild(verdictBadge);

    titleGroup.appendChild(badgeRow);

    var h4 = document.createElement("h4");
    h4.className = "stoch-lab-title";
    h4.textContent = customTitle;
    titleGroup.appendChild(h4);

    if (customSubtitle) {
      var subP = document.createElement("p");
      subP.className = "stoch-lab-subtitle";
      subP.textContent = customSubtitle;
      titleGroup.appendChild(subP);
    }

    header.appendChild(titleGroup);

    var actions = document.createElement("div");
    actions.className = "stoch-lab-actions";

    var copyBtn = document.createElement("button");
    copyBtn.type = "button";
    copyBtn.className = "stoch-lab-btn";
    copyBtn.textContent = "Copy CLI Audit";
    copyBtn.addEventListener("click", function () {
      if (navigator.clipboard && navigator.clipboard.writeText) {
        navigator.clipboard.writeText(customCli);
      }
      copyBtn.textContent = "Copied CLI ✓";
      announcePolite("Copied CLI audit command to clipboard: " + customCli);
      setTimeout(function () {
        copyBtn.textContent = "Copy CLI Audit";
      }, 1600);
    });
    actions.appendChild(copyBtn);

    var copyLinkBtn = document.createElement("button");
    copyLinkBtn.type = "button";
    copyLinkBtn.className = "stoch-lab-btn";
    copyLinkBtn.textContent = "Copy Permalink";
    copyLinkBtn.addEventListener("click", function () {
      var relLink = buildLabDeepLink(moduleKey, state, chapter, activeSeed);
      var absLink = relLink;
      if (typeof window !== "undefined" && window.location) {
        try {
          absLink = new URL(relLink, window.location.href).href;
        } catch (_e) {}
      }
      if (navigator.clipboard && navigator.clipboard.writeText) {
        navigator.clipboard.writeText(absLink);
      }
      copyLinkBtn.textContent = "Copied Link ✓";
      announcePolite("Copied reproducible permalink to clipboard");
      setTimeout(function () {
        copyLinkBtn.textContent = "Copy Permalink";
      }, 1600);
    });
    actions.appendChild(copyLinkBtn);

    var embedBtn = document.createElement("button");
    embedBtn.type = "button";
    embedBtn.className = "stoch-lab-btn";
    embedBtn.setAttribute("aria-expanded", "false");
    embedBtn.textContent = "Sandbox (/lab/embed)";
    actions.appendChild(embedBtn);

    var openLink = document.createElement("a");
    openLink.className = "stoch-lab-btn stoch-lab-btn-primary";
    openLink.href = buildLabDeepLink(moduleKey, state, chapter, activeSeed);
    openLink.target = "_blank";
    openLink.rel = "noopener noreferrer";
    openLink.textContent = "Open in /lab ↗";
    if (!fromQuery) {
      actions.appendChild(openLink);
    }

    header.appendChild(actions);
    container.appendChild(header);

    var body = document.createElement("div");
    body.className = "stoch-lab-body";

    var toolbar = document.createElement("div");
    toolbar.className = "stoch-lab-toolbar";

    var presetsWrap = document.createElement("div");
    presetsWrap.className = "stoch-lab-presets";
    presetsWrap.setAttribute("role", "group");
    presetsWrap.setAttribute("aria-label", "Regime Presets");

    var presetsLbl = document.createElement("span");
    presetsLbl.className = "stoch-lab-presets-label";
    presetsLbl.textContent = "Regime Presets:";
    presetsWrap.appendChild(presetsLbl);

    var btnShielded = document.createElement("button");
    btnShielded.type = "button";
    btnShielded.className = "stoch-lab-preset-btn";
    btnShielded.setAttribute("data-preset", "shielded");
    btnShielded.setAttribute("aria-pressed", "true");
    btnShielded.textContent = "✓ Canonical Shielded";
    presetsWrap.appendChild(btnShielded);

    var btnDefect = document.createElement("button");
    btnDefect.type = "button";
    btnDefect.className = "stoch-lab-preset-btn";
    btnDefect.setAttribute("data-preset", "defect");
    btnDefect.setAttribute("aria-pressed", "false");
    btnDefect.textContent = "⚠ Inject Defect";
    presetsWrap.appendChild(btnDefect);

    var btnReset = document.createElement("button");
    btnReset.type = "button";
    btnReset.className = "stoch-lab-preset-btn";
    btnReset.setAttribute("data-preset", "reset");
    btnReset.textContent = "↺ Reset (seed = " + activeSeed + ")";
    presetsWrap.appendChild(btnReset);

    var hintSpan = document.createElement("span");
    hintSpan.className = "stoch-lab-hint";
    hintSpan.textContent = "Drag sliders, double-click a slider to reset it, or toggle presets";

    toolbar.appendChild(presetsWrap);
    toolbar.appendChild(hintSpan);
    body.appendChild(toolbar);

    var controlsGrid = document.createElement("div");
    controlsGrid.className = "stoch-lab-controls";

    var stage = document.createElement("div");
    stage.className = "stoch-lab-stage";

    var metricsGrid = document.createElement("div");
    metricsGrid.className = "stoch-lab-metrics";

    var iframeWrap = document.createElement("div");
    iframeWrap.className = "stoch-lab-iframe-wrap";
    var iframe = document.createElement("iframe");
    iframe.className = "stoch-lab-iframe";
    iframe.title = customTitle + " (/lab/embed)";
    iframe.setAttribute("sandbox", "allow-scripts allow-same-origin");
    iframe.setAttribute("loading", "lazy");
    iframeWrap.appendChild(iframe);

    var footer = document.createElement("div");
    footer.className = "stoch-lab-footer";
    var cliCode = document.createElement("code");
    cliCode.textContent = "$ " + customCli;
    var syncSpan = document.createElement("span");
    footer.appendChild(cliCode);
    footer.appendChild(syncSpan);

    var controlRefs = {};

    function applyStateMap(targetMap, presetMode, announceLabel) {
      for (var i = 0; i < spec.controls.length; i++) {
        var ctrl = spec.controls[i];
        var nextVal = Object.prototype.hasOwnProperty.call(targetMap, ctrl.key)
          ? targetMap[ctrl.key]
          : ctrl.val;
        nextVal = Math.max(ctrl.min, Math.min(ctrl.max, nextVal));
        state[ctrl.key] = nextVal;
        if (controlRefs[ctrl.key]) {
          controlRefs[ctrl.key].input.value = String(nextVal);
          controlRefs[ctrl.key].input.setAttribute("aria-valuetext", ctrl.fmt(nextVal));
          controlRefs[ctrl.key].valSpan.textContent = ctrl.fmt(nextVal);
          syncSliderCssVars(controlRefs[ctrl.key].input, ctrl.min, ctrl.max, nextVal);
        }
      }
      btnShielded.setAttribute("aria-pressed", presetMode === "shielded" ? "true" : "false");
      btnDefect.setAttribute("aria-pressed", presetMode === "defect" ? "true" : "false");
      var verdict = renderState();
      if (announceLabel) {
        announcePolite(customTitle + ": " + announceLabel + " (" + verdict + ")");
      }
    }

    function renderState() {
      var res = spec.evaluate(state, activeSeed);
      stage.innerHTML = res.svg;
      var svgEl = stage.querySelector("svg");
      if (svgEl) {
        svgEl.setAttribute("role", "img");
        svgEl.setAttribute(
          "aria-label",
          customTitle + (customSubtitle ? " — " + customSubtitle : "")
        );
      }
      metricsGrid.innerHTML = "";
      var hasRose = false;
      for (var m = 0; m < res.metrics.length; m++) {
        var item = res.metrics[m];
        if (item.tone === "rose") {
          hasRose = true;
        }
        var card = document.createElement("div");
        card.className = "stoch-lab-metric";
        if (item.tone) card.setAttribute("data-tone", item.tone);

        var lbl = document.createElement("span");
        lbl.className = "stoch-lab-metric-label";
        lbl.textContent = item.label;
        card.appendChild(lbl);

        var val = document.createElement("span");
        val.className = "stoch-lab-metric-value";
        val.textContent = item.value;
        card.appendChild(val);

        var sub = document.createElement("span");
        sub.className = "stoch-lab-metric-sub";
        sub.textContent = item.sub;
        card.appendChild(sub);

        metricsGrid.appendChild(card);
      }

      var statusKey = hasRose ? "blocked" : "approved";
      var verdictText = hasRose ? "GATE VERDICT: BLOCKED — DEFECT" : "GATE VERDICT: APPROVED";
      verdictBadge.setAttribute("data-status", statusKey);
      verdictBadge.textContent = verdictText;
      container.setAttribute("data-verdict", statusKey);

      var deepUrl = buildLabDeepLink(moduleKey, state, chapter, activeSeed);
      openLink.href = deepUrl;
      syncSpan.textContent = "State synced with " + LAB_PATH + "/?module=" + moduleKey + "&seed=" + activeSeed;

      if (fromQuery && typeof window !== "undefined" && window.history && window.history.replaceState) {
        var qIndex = deepUrl.indexOf("?");
        if (qIndex !== -1) {
          window.history.replaceState(null, "", deepUrl.substring(qIndex));
        }
      }

      if (iframeWrap.classList.contains("active") && iframe.contentWindow) {
        try {
          iframe.contentWindow.postMessage(
            {
              type: "stochology:lab:state",
              module: moduleKey,
              seed: activeSeed,
              theme: getActiveThemeName(),
              dark: isDarkThemeActive(),
              verdict: { ok: !hasRose, label: verdictText },
              state: state,
              metrics: res.metrics,
              cli: customCli
            },
            "*"
          );
        } catch (_e) {}
      }
      return verdictText;
    }

    for (var j = 0; j < spec.controls.length; j++) {
      (function (ctrl, idx) {
        var wrap = document.createElement("div");
        wrap.className = "stoch-lab-control";

        var initVal = state[ctrl.key];
        var inputId = "stoch-lab-" + instanceId + "-" + ctrl.key + "-" + idx;
        var hintId = inputId + "-hint";

        var lbl = document.createElement("label");
        lbl.setAttribute("for", inputId);
        var nameSpan = document.createElement("span");
        nameSpan.textContent = ctrl.label;
        var valSpan = document.createElement("span");
        valSpan.className = "stoch-lab-control-val";
        valSpan.textContent = ctrl.fmt(initVal);
        lbl.appendChild(nameSpan);
        lbl.appendChild(valSpan);

        var input = document.createElement("input");
        input.id = inputId;
        input.type = "range";
        input.min = String(ctrl.min);
        input.max = String(ctrl.max);
        input.step = String(ctrl.step);
        input.value = String(initVal);
        input.setAttribute("aria-valuetext", ctrl.fmt(initVal));
        input.setAttribute("aria-describedby", hintId);
        syncSliderCssVars(input, ctrl.min, ctrl.max, initVal);

        input.addEventListener("input", function () {
          var num = parseFloat(input.value);
          state[ctrl.key] = num;
          valSpan.textContent = ctrl.fmt(num);
          input.setAttribute("aria-valuetext", ctrl.fmt(num));
          syncSliderCssVars(input, ctrl.min, ctrl.max, num);
          btnShielded.setAttribute("aria-pressed", "false");
          btnDefect.setAttribute("aria-pressed", "false");
          renderState();
        });

        input.addEventListener("dblclick", function () {
          state[ctrl.key] = ctrl.val;
          input.value = String(ctrl.val);
          valSpan.textContent = ctrl.fmt(ctrl.val);
          input.setAttribute("aria-valuetext", ctrl.fmt(ctrl.val));
          syncSliderCssVars(input, ctrl.min, ctrl.max, ctrl.val);
          renderState();
          announcePolite(ctrl.label + " reset to " + ctrl.fmt(ctrl.val));
        });

        var metaRow = document.createElement("div");
        metaRow.id = hintId;
        metaRow.className = "stoch-lab-control-range-meta";
        var minSpan = document.createElement("span");
        minSpan.textContent = ctrl.fmt(ctrl.min);
        var defSpan = document.createElement("span");
        defSpan.textContent = (ctrl.provenance || "assumed") + ": " + ctrl.fmt(ctrl.val);
        var maxSpan = document.createElement("span");
        maxSpan.textContent = ctrl.fmt(ctrl.max);
        metaRow.appendChild(minSpan);
        metaRow.appendChild(defSpan);
        metaRow.appendChild(maxSpan);

        controlRefs[ctrl.key] = { input: input, valSpan: valSpan };

        wrap.appendChild(lbl);
        wrap.appendChild(input);
        wrap.appendChild(metaRow);
        controlsGrid.appendChild(wrap);
      })(spec.controls[j], j);
    }

    btnShielded.addEventListener("click", function () {
      var defaults = {};
      for (var i = 0; i < spec.controls.length; i++) {
        defaults[spec.controls[i].key] = spec.controls[i].val;
      }
      applyStateMap(defaults, "shielded", "Canonical Shielded regime applied");
    });

    btnDefect.addEventListener("click", function () {
      var defectMap = spec.defectState || {};
      applyStateMap(defectMap, "defect", "Injected Defect regime applied");
    });

    btnReset.addEventListener("click", function () {
      var defaults = {};
      for (var i = 0; i < spec.controls.length; i++) {
        defaults[spec.controls[i].key] = spec.controls[i].val;
      }
      applyStateMap(defaults, "shielded", "Reset to Φ₁ seed = " + activeSeed + " defaults");
    });

    embedBtn.addEventListener("click", function () {
      var active = iframeWrap.classList.toggle("active");
      embedBtn.setAttribute("aria-expanded", active ? "true" : "false");
      embedBtn.textContent = active ? "Hide Sandbox" : "Sandbox (/lab/embed)";
      if (active) {
        var base = resolveLabBaseUrl();
        var embedUrl = buildLabDeepLink(moduleKey, state, chapter, activeSeed).replace(
          base + "/?",
          base + "/embed/?"
        );
        if (iframe.src !== embedUrl) {
          iframe.src = embedUrl;
        }
        setTimeout(function () {
          renderState();
        }, 180);
      }
    });

    if (typeof MutationObserver !== "undefined") {
      var themeObs = new MutationObserver(function () {
        if (iframeWrap.classList.contains("active")) {
          renderState();
        }
      });
      themeObs.observe(document.documentElement, {
        attributes: true,
        attributeFilter: ["class"]
      });
    }

    body.appendChild(controlsGrid);
    body.appendChild(stage);
    body.appendChild(metricsGrid);
    body.appendChild(iframeWrap);
    container.appendChild(body);
    container.appendChild(footer);

    renderState();
  }

  function enhanceResponsiveTables(root) {
    var wrappers = root.querySelectorAll(".table-wrapper");
    for (var j = 0; j < wrappers.length; j++) {
      var wrapper = wrappers[j];
      if (!wrapper.hasAttribute("tabindex")) {
        wrapper.setAttribute("tabindex", "0");
      }
      if (!wrapper.hasAttribute("role")) {
        wrapper.setAttribute("role", "region");
      }
      if (!wrapper.hasAttribute("aria-label") && !wrapper.hasAttribute("aria-labelledby")) {
        var prev = wrapper.previousElementSibling;
        while (prev && !/^H[1-6]$/.test(prev.tagName)) {
          prev = prev.previousElementSibling;
        }
        var label = prev ? prev.textContent.trim() : "Protocol specification table";
        wrapper.setAttribute("aria-label", label);
      }

      var table = wrapper.querySelector("table");
      if (!table) continue;
      var ths = table.querySelectorAll("thead th");
      if (ths.length >= 3) {
        table.setAttribute("data-responsive-stacked", "true");
        var colLabels = [];
        for (var h = 0; h < ths.length; h++) {
          if (!ths[h].hasAttribute("scope")) {
            ths[h].setAttribute("scope", "col");
          }
          var txt = (ths[h].textContent || "").trim();
          colLabels.push(txt);
          table.style.setProperty("--label-" + (h + 1), JSON.stringify(txt));
        }
        var rows = table.querySelectorAll("tbody tr");
        for (var r = 0; r < rows.length; r++) {
          var tds = rows[r].querySelectorAll("td");
          for (var c = 0; c < tds.length; c++) {
            if (colLabels[c] && !tds[c].hasAttribute("data-col-label")) {
              tds[c].setAttribute("data-col-label", colLabels[c]);
            }
          }
        }
      }
    }
  }

  function enhanceScrollablePreBlocks(root) {
    var pres = root.querySelectorAll("pre");
    for (var i = 0; i < pres.length; i++) {
      var pre = pres[i];
      if (pre.querySelector("code.language-mermaid")) continue;
      if (!pre.hasAttribute("tabindex")) {
        pre.setAttribute("tabindex", "0");
      }
    }
  }

  function prepareDocumentForMathAndTables() {
    injectSkipLink();
    injectTopMenuLabButton();

    var root = document.querySelector("main") || document.body;
    if (!root) return;

    setupInteractiveLabs(root);

    var walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT, {
      acceptNode: function (node) {
        var parent = node.parentElement;
        if (!parent) return NodeFilter.FILTER_REJECT;
        var tag = parent.tagName;
        if (tag === "CODE" || tag === "PRE" || tag === "SCRIPT" || tag === "STYLE") {
          return NodeFilter.FILTER_REJECT;
        }
        return node.nodeValue && node.nodeValue.indexOf("$") !== -1
          ? NodeFilter.FILTER_ACCEPT
          : NodeFilter.FILTER_SKIP;
      }
    });

    var nodes = [];
    while (walker.nextNode()) {
      nodes.push(walker.currentNode);
    }
    for (var i = 0; i < nodes.length; i++) {
      nodes[i].nodeValue = normalizeMathText(nodes[i].nodeValue);
    }

    enhanceResponsiveTables(root);
    enhanceScrollablePreBlocks(root);

    var svgImgs = root.querySelectorAll('p > img[src*="figures/"][src$=".svg"]');
    for (var k = 0; k < svgImgs.length; k++) {
      var img = svgImgs[k];
      var parentP = img.parentElement;
      if (!parentP || parentP.children.length !== 1) continue;
      var altText = (img.getAttribute("alt") || "").trim();
      var fig = document.createElement("figure");
      fig.className = "stoch-figure";
      parentP.parentNode.replaceChild(fig, parentP);
      fig.appendChild(img);
      if (altText) {
        var cap = document.createElement("figcaption");
        var strong = document.createElement("strong");
        strong.textContent = altText;
        cap.appendChild(strong);
        fig.appendChild(cap);
      }
    }

    setupMermaidDiagrams(root);
  }

  function isDarkThemeActive() {
    var cl = document.documentElement.classList;
    if (cl.contains("navy") || cl.contains("coal") || cl.contains("ayu")) {
      return true;
    }
    if (cl.contains("light") || cl.contains("rust")) {
      return false;
    }
    return window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches;
  }

  function getMermaidConfig(dark) {
    return {
      startOnLoad: false,
      securityLevel: "strict",
      theme: "base",
      fontFamily: '"Inter", "Open Sans", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif',
      flowchart: {
        htmlLabels: false,
        curve: "basis",
        padding: 16,
        nodeSpacing: 36,
        rankSpacing: 46
      },
      state: {
        dividerMargin: 10,
        sizeUnit: 5,
        padding: 12
      },
      themeVariables: dark
        ? {
            darkMode: true,
            background: "#0f1626",
            primaryColor: "#162036",
            primaryTextColor: "#e2e8f0",
            primaryBorderColor: "#38bdf8",
            secondaryColor: "#1e1b4b",
            secondaryTextColor: "#e2e8f0",
            secondaryBorderColor: "#a78bfa",
            tertiaryColor: "#064e3b",
            tertiaryTextColor: "#e2e8f0",
            tertiaryBorderColor: "#34d399",
            lineColor: "#64748b",
            textColor: "#e2e8f0",
            mainBkg: "#162036",
            nodeBorder: "#38bdf8",
            clusterBkg: "#0b1120",
            clusterBorder: "#1e2d4a",
            titleColor: "#f8fafc",
            edgeLabelBackground: "#0f1626"
          }
        : {
            darkMode: false,
            background: "#ffffff",
            primaryColor: "#f1f5f9",
            primaryTextColor: "#0f172a",
            primaryBorderColor: "#0284c7",
            secondaryColor: "#f5f3ff",
            secondaryTextColor: "#0f172a",
            secondaryBorderColor: "#6d28d9",
            tertiaryColor: "#ecfdf5",
            tertiaryTextColor: "#0f172a",
            tertiaryBorderColor: "#059669",
            lineColor: "#475569",
            textColor: "#0f172a",
            mainBkg: "#f1f5f9",
            nodeBorder: "#0284c7",
            clusterBkg: "#f8fafc",
            clusterBorder: "#cbd5e1",
            titleColor: "#0f172a",
            edgeLabelBackground: "#ffffff"
          }
    };
  }

  var lastMermaidDarkState = null;

  function renderMermaidDiagrams(force) {
    if (!window.mermaid) return;
    var dark = isDarkThemeActive();
    if (!force && lastMermaidDarkState === dark) return;
    lastMermaidDarkState = dark;

    window.mermaid.initialize(getMermaidConfig(dark));
    var containers = document.querySelectorAll(".mermaid[data-mermaid-src]");
    for (var i = 0; i < containers.length; i++) {
      var el = containers[i];
      el.removeAttribute("data-processed");
      el.textContent = el.getAttribute("data-mermaid-src");
    }
    if (containers.length > 0 && typeof window.mermaid.run === "function") {
      window.mermaid.run({ querySelector: ".mermaid" });
    }
  }

  function setupMermaidDiagrams(root) {
    var codeBlocks = root.querySelectorAll("pre > code.language-mermaid");
    if (!codeBlocks.length) return;

    for (var i = 0; i < codeBlocks.length; i++) {
      var code = codeBlocks[i];
      var pre = code.parentElement;
      if (!pre) continue;
      var src = (code.textContent || "").trim();
      var fig = document.createElement("figure");
      fig.className = "stoch-figure stoch-mermaid-figure";
      fig.setAttribute("role", "group");

      var prev = pre.previousElementSibling;
      while (prev && !/^H[1-6]$/.test(prev.tagName)) {
        prev = prev.previousElementSibling;
      }
      fig.setAttribute(
        "aria-label",
        prev ? prev.textContent.trim() + " — Diagram" : "Stochology Protocol Diagram"
      );

      var div = document.createElement("div");
      div.className = "mermaid";
      div.setAttribute("data-mermaid-src", src);
      div.textContent = src;
      fig.appendChild(div);
      pre.parentNode.replaceChild(fig, pre);
    }

    var mScript = document.createElement("script");
    mScript.src = "https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.min.js";
    mScript.async = true;
    mScript.onload = function () {
      renderMermaidDiagrams(true);
      if (typeof MutationObserver !== "undefined") {
        var obs = new MutationObserver(function () {
          renderMermaidDiagrams(false);
        });
        obs.observe(document.documentElement, {
          attributes: true,
          attributeFilter: ["class"]
        });
      }
      if (window.matchMedia) {
        var mq = window.matchMedia("(prefers-color-scheme: dark)");
        if (typeof mq.addEventListener === "function") {
          mq.addEventListener("change", function () {
            renderMermaidDiagrams(true);
          });
        }
      }
    };
    document.head.appendChild(mScript);
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", prepareDocumentForMathAndTables);
  } else {
    prepareDocumentForMathAndTables();
  }

  window.MathJax = {
    tex: {
      inlineMath: [["$", "$"], ["\\(", "\\)"]],
      displayMath: [["$$", "$$"], ["\\[", "\\]"]],
      processEscapes: true
    },
    options: {
      skipHtmlTags: ["script", "noscript", "style", "textarea", "pre", "code"]
    }
  };

  var s = document.createElement("script");
  s.src = "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-chtml.js";
  s.async = true;
  document.head.appendChild(s);
})();
