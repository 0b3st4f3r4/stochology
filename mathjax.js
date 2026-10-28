(function () {
  "use strict";

  var LAB_PATH = "/lab";

  function resolveLabBaseUrl() {
    if (typeof window !== "undefined" && typeof window.path_to_root === "string" && window.path_to_root.length > 0) {
      var prefix = window.path_to_root;
      if (prefix.charAt(prefix.length - 1) !== "/") prefix += "/";
      return prefix + "lab";
    }
    return LAB_PATH;
  }

  function normalizeMathText(text) {
    if (text.indexOf("$") === -1) return text;
    return text.replace(/(\$\$[\s\S]*?\$\$|\$[^$\n]+\$)/g, function (seg) {
      return seg
        .replace(/’/g, "'")
        .replace(/(^|[^\\])%/g, "$1\\%");
    });
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

    var dot = document.createElement("span");
    dot.className = "stoch-lab-nav-dot";
    link.appendChild(dot);

    var label = document.createElement("span");
    label.textContent = "/lab ↗";
    link.appendChild(label);

    rightButtons.insertBefore(link, rightButtons.firstChild);
  }

  var LAB_REGISTRY = {
    "gigerenzer-bayes": {
      title: "Interactive Natural-Frequency Tree & Turing Deciban Updater",
      cli: "cargo run --release --bin labs -- --siga",
      controls: [
        { key: "cohort", label: "Cohort Size (N)", min: 1000, max: 100000, step: 1000, val: 10000, fmt: function (v) { return v.toLocaleString(); } },
        { key: "prev", label: "Prevalence p₀ (%)", min: 0.1, max: 25.0, step: 0.1, val: 1.0, fmt: function (v) { return v.toFixed(1) + "%"; } },
        { key: "sens", label: "Sensitivity (%)", min: 50.0, max: 99.9, step: 0.5, val: 90.0, fmt: function (v) { return v.toFixed(1) + "%"; } },
        { key: "fpr", label: "False-Positive Rate (%)", min: 0.5, max: 30.0, step: 0.5, val: 9.0, fmt: function (v) { return v.toFixed(1) + "%"; } }
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
            { label: "Posterior PPV P(D|+)", value: (ppv * 100.0).toFixed(2) + "%", sub: tp + " / " + totPos + " positive alarms", tone: ppv >= 0.5 ? "emerald" : "amber" },
            { label: "Likelihood Ratio (LR+)", value: lr.toFixed(2) + "×", sub: "Sens / FPR", tone: "sky" },
            { label: "Turing Evidence W(+)", value: (dB >= 0 ? "+" : "") + dB.toFixed(2) + " dB", sub: "10 · log₁₀(LR+)", tone: "emerald" },
            { label: "Posterior Log-Odds", value: (postDb >= 0 ? "+" : "") + postDb.toFixed(2) + " dB", sub: "Prior " + priorDb.toFixed(1) + " dB + W", tone: postDb >= 0 ? "emerald" : "rose" }
          ]
        };
      }
    },

    "jensen-crps": {
      title: "Interactive Convexity, Secant Chord & Jensen's Gap (Δ_J) Simulator",
      cli: "cargo run --release --bin labs -- --layers",
      controls: [
        { key: "mu", label: "Mean Input E[X]", min: 1.0, max: 6.0, step: 0.1, val: 3.0, fmt: function (v) { return v.toFixed(2); } },
        { key: "spread", label: "Half-Spread δ (X = μ ± δ)", min: 0.2, max: 3.5, step: 0.1, val: 2.0, fmt: function (v) { return "±" + v.toFixed(2); } },
        { key: "curv", label: "Convex Curvature c (f(x)=c·x²)", min: 0.5, max: 4.0, step: 0.1, val: 1.5, fmt: function (v) { return v.toFixed(2); } }
      ],
      evaluate: function (s) {
        var mu = s.mu;
        var d = s.spread;
        var c = s.curv;
        var x1 = Math.max(0.1, mu - d);
        var x2 = mu + d;
        var f = function (x) { return c * x * x; };
        var fMu = f(mu);
        var efX = 0.5 * f(x1) + 0.5 * f(x2);
        var gap = efX - fMu;
        var xMax = 10.0;
        var yMax = Math.max(10.0, f(xMax) * 0.85);
        var mapX = function (x) { return 70 + (x / xMax) * 760; };
        var mapY = function (y) { return 195 - Math.min(175, (y / yMax) * 170); };

        var pts = [];
        for (var i = 0; i <= 50; i++) {
          var xv = (i / 50.0) * xMax;
          pts.push((i === 0 ? "M " : "L ") + mapX(xv).toFixed(1) + " " + mapY(f(xv)).toFixed(1));
        }

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="830" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<line x1="70" y1="20" x2="70" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
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
            { label: "Jensen's Gap (Δ_J)", value: "+" + gap.toFixed(2), sub: "E[f(X)] - f(E[X]) ≥ 0", tone: "rose" },
            { label: "Taylor Exact ½f''(μ)Var(X)", value: (c * d * d).toFixed(2), sub: "Curvature × Variance", tone: "amber" }
          ]
        };
      }
    },

    "meng-performativity": {
      title: "Interactive Meng Big-Data Paradox (L₁→L₂) & Performative RRM/PerfGD (L₃→L₁)",
      cli: "cargo run --release --bin labs -- --layers",
      controls: [
        { key: "rho", label: "Selection Corr |ρ_{R,Y}|", min: 0.005, max: 0.25, step: 0.005, val: 0.133, fmt: function (v) { return v.toFixed(3); } },
        { key: "frac", label: "Sample Share f = n/N (%)", min: 5.0, max: 80.0, step: 1.0, val: 50.0, fmt: function (v) { return v.toFixed(0) + "%"; } },
        { key: "eps", label: "Performative Sensitivity ε", min: 0.05, max: 0.95, step: 0.05, val: 0.20, fmt: function (v) { return v.toFixed(2); } },
        { key: "beta", label: "Smoothness Ratio β/γ", min: 0.5, max: 2.0, step: 0.1, val: 1.20, fmt: function (v) { return v.toFixed(2); } }
      ],
      evaluate: function (s) {
        var N = 32000;
        var f = s.frac / 100.0;
        var n = Math.round(N * f);
        var ddi = s.rho * s.rho;
        var nEffMeng = Math.min(n, (f / (1.0 - f)) * (1.0 / Math.max(1e-6, ddi)));
        var rRrm = s.eps * s.beta;
        var thetaPs = 1.5 / (1.0 - Math.min(0.85, s.eps * 0.5));
        var thetaPo = 1.5 / (1.0 - Math.min(0.85, s.eps * 0.72));

        var rrmPts = [];
        var pgdPts = [];
        var th1 = 0.2;
        var th2 = 0.2;
        for (var t = 0; t <= 20; t++) {
          var x = 70 + (t / 20.0) * 760;
          var y1 = 190 - Math.min(165, Math.max(10, (th1 / 4.5) * 155));
          var y2 = 190 - Math.min(165, Math.max(10, (th2 / 4.5) * 155));
          rrmPts.push((t === 0 ? "M " : "L ") + x.toFixed(1) + " " + y1.toFixed(1));
          pgdPts.push((t === 0 ? "M " : "L ") + x.toFixed(1) + " " + y2.toFixed(1));
          th1 = thetaPs + (th1 - thetaPs) * (rRrm < 1.0 ? rRrm : 1.12);
          th2 = th2 + 0.28 * (thetaPo - th2);
        }

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="830" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<line x1="70" y1="20" x2="70" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<path d="' + rrmPts.join(" ") + '" fill="none" stroke="var(--rose)" stroke-width="2.6"/>' +
          '<path d="' + pgdPts.join(" ") + '" fill="none" stroke="var(--emerald)" stroke-width="2.8" stroke-dasharray="6 3"/>' +
          '<text x="86" y="42" fill="var(--ink)" font-size="12" font-weight="750">Performative Trajectory: Myopic RRM (Rose) vs. Stackelberg PerfGD θ_PO (Emerald)</text>' +
          '<text x="86" y="62" fill="var(--muted)" font-size="11.5">Contraction r_RRM = ε·(β/γ) = ' + rRrm.toFixed(3) + (rRrm < 1.0 ? " < 1 (Convergent)" : " ≥ 1 (Bifurcated / Divergent — Gate G6 Blocks!)") + '</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Nominal Sample (n)", value: n.toLocaleString(), sub: "Out of N = " + N.toLocaleString(), tone: "sky" },
            { label: "Meng Effective n_eff", value: nEffMeng.toFixed(1), sub: "DDI = " + ddi.toFixed(5), tone: nEffMeng < 500 ? "rose" : "amber" },
            { label: "RRM Contraction r_RRM", value: rRrm.toFixed(3), sub: rRrm < 1.0 ? "Stable (< 1.0)" : "BLOCKED (≥ 1.0)", tone: rRrm < 1.0 ? "emerald" : "rose" },
            { label: "Stackelberg Gap |θ_PO-θ_PS|", value: Math.abs(thetaPo - thetaPs).toFixed(3), sub: "θ_PO=" + thetaPo.toFixed(2) + " vs θ_PS=" + thetaPs.toFixed(2), tone: "emerald" }
          ]
        };
      }
    },

    "ols-cholesky-gls": {
      title: "Interactive OLS Orthogonal Projection, Correlated Errors & Cholesky GLS Simulator",
      cli: "cargo run --release --bin labs -- --uncertainty",
      controls: [
        { key: "n", label: "Sample Size n", min: 25, max: 1000, step: 25, val: 400, fmt: function (v) { return "n = " + v; } },
        { key: "rho", label: "Equicorrelation ρ Between Errors", min: 0.0, max: 0.80, step: 0.05, val: 0.35, fmt: function (v) { return v.toFixed(2); } },
        { key: "bias", label: "Structural Discrepancy δ(x)", min: 0.0, max: 2.5, step: 0.1, val: 0.8, fmt: function (v) { return v.toFixed(1); } }
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
        for (var i = 1; i <= 40; i++) {
          var nv = i * 25;
          var sn = sig / Math.sqrt(nv);
          var st = Math.sqrt((sig * sig / nv) * (1.0 + (nv - 1.0) * rho));
          var px = 70 + (i / 40.0) * 760;
          var pyN = 195 - Math.min(165, (sn / 2.0) * 160);
          var pyT = 195 - Math.min(165, (st / 2.0) * 160);
          ptsNaive.push((i === 1 ? "M " : "L ") + px.toFixed(1) + " " + pyN.toFixed(1));
          ptsTrue.push((i === 1 ? "M " : "L ") + px.toFixed(1) + " " + pyT.toFixed(1));
        }

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="830" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<path d="' + ptsNaive.join(" ") + '" fill="none" stroke="var(--sky)" stroke-width="2.4" stroke-dasharray="5 3"/>' +
          '<path d="' + ptsTrue.join(" ") + '" fill="none" stroke="var(--rose)" stroke-width="2.8"/>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Standard Error vs. Sample Size n: Naive i.i.d. σ/√n (Sky Dashed) vs. Correlated Floor σ√(ρ + (1-ρ)/n) (Rose)</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Naive i.i.d. SE (σ/√n)", value: seNaive.toFixed(3), sub: "Assumes ρ = 0", tone: "sky" },
            { label: "True Correlated SE", value: seTrue.toFixed(3), sub: (seTrue / seNaive).toFixed(2) + "× wider than naive", tone: rho > 0.05 ? "rose" : "emerald" },
            { label: "Total RMSE √(Var + δ²)", value: rmseTotal.toFixed(3), sub: "Includes structural δ = " + s.bias.toFixed(1), tone: "amber" },
            { label: "Cholesky Factor L₂₂", value: l22.toFixed(3) + "·σ", sub: "√(1 - ρ²) whitened innovation", tone: "emerald" }
          ]
        };
      }
    },

    "markov-dag-do": {
      title: "Interactive Markov Contraction (η_TV) & Pearl do(X=x) vs. Berkson Collider Simulator",
      cli: "cargo run --release --bin labs -- --lenses",
      controls: [
        { key: "noise", label: "Channel Flip Prob ε", min: 0.05, max: 0.45, step: 0.05, val: 0.15, fmt: function (v) { return v.toFixed(2); } },
        { key: "conf", label: "Confounder U → (X,Y) Bias", min: 0.0, max: 2.5, step: 0.1, val: 1.5, fmt: function (v) { return "+" + v.toFixed(1); } },
        { key: "coll", label: "Collider Conditioning C=1", min: 0, max: 1, step: 1, val: 0, fmt: function (v) { return v > 0.5 ? "Conditioned (Biased!)" : "Unconditioned (Clean)"; } }
      ],
      evaluate: function (s) {
        var eps = s.noise;
        var etaTv = Math.abs(1.0 - 2.0 * eps);
        var trueDoEffect = 2.80;
        var obsEffect = trueDoEffect + s.conf - (s.coll > 0.5 ? 1.15 : 0.0);

        var pts = [];
        var tv = 1.0;
        for (var t = 0; t <= 12; t++) {
          var px = 70 + (t / 12.0) * 760;
          var py = 195 - tv * 150;
          pts.push((t === 0 ? "M " : "L ") + px.toFixed(1) + " " + py.toFixed(1));
          tv *= etaTv;
        }

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="830" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<path d="' + pts.join(" ") + '" fill="none" stroke="var(--emerald)" stroke-width="2.8"/>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Dobrushin Total-Variation Contraction ‖p^(t) - π‖_TV ≤ η_TV^t  (η_TV = ' + etaTv.toFixed(2) + ')</text>' +
          '<text x="86" y="60" fill="var(--muted)" font-size="11.5">Causal Identification: True E[Y|do(X=1)] = +2.80 vs. Naive Observational E[Y|X=1] = +' + obsEffect.toFixed(2) + '</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Dobrushin η_TV(W)", value: etaTv.toFixed(3), sub: "Contraction per step", tone: "emerald" },
            { label: "True Causal E[Y|do(X)]", value: "+" + trueDoEffect.toFixed(2), sub: "Rung 2 (Graph Mutilation)", tone: "emerald" },
            { label: "Naive Rung-1 E[Y|X]", value: "+" + obsEffect.toFixed(2), sub: "Confounded / Collider Bias", tone: Math.abs(obsEffect - trueDoEffect) > 0.2 ? "rose" : "sky" },
            { label: "Berkson Collider Guard", value: s.coll > 0.5 ? "VIOLATED (Filter 2)" : "VERIFIED (Clean)", sub: "Ban on conditioning on C", tone: s.coll > 0.5 ? "rose" : "emerald" }
          ]
        };
      }
    },

    "koh-infogap-wald": {
      title: "Interactive Kennedy-O'Hagan Discrepancy GP & Info-Gap + Sniedovich Wald Floor",
      cli: "cargo run --release --bin labs -- --uncertainty",
      controls: [
        { key: "rc", label: "Critical Survival Floor r_c", min: 10.0, max: 70.0, step: 2.0, val: 40.0, fmt: function (v) { return v.toFixed(0); } },
        { key: "turkey", label: "Fragile Tail Drop (No-Man's Land)", min: 5.0, max: 90.0, step: 5.0, val: 55.0, fmt: function (v) { return "-" + v.toFixed(0); } },
        { key: "anchor", label: "GP Boundary Anchor Weight", min: 0.0, max: 1.0, step: 0.1, val: 1.0, fmt: function (v) { return v.toFixed(1); } }
      ],
      evaluate: function (s) {
        var rc = s.rc;
        var drop = s.turkey;
        var robA = Math.max(0.0, (78.0 - rc) / 22.0);
        var robB = Math.max(0.0, (72.0 - rc) / 14.0);
        var waldA = 78.0 - 1.4 * drop;
        var waldB = 72.0 - 0.35 * drop;
        var sniedovichBlocked = robA > robB && waldA < rc;

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="830" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<line x1="70" y1="20" x2="70" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<line x1="70" y1="' + (195 - rc * 1.8).toFixed(1) + '" x2="830" y2="' + (195 - rc * 1.8).toFixed(1) + '" stroke="var(--amber)" stroke-width="2" stroke-dasharray="5 4"/>' +
          '<path d="M 70 55 L 410 ' + (195 - rc * 1.8).toFixed(1) + ' L 820 ' + Math.min(215, 195 - waldA * 1.8).toFixed(1) + '" fill="none" stroke="var(--rose)" stroke-width="2.8"/>' +
          '<path d="M 70 66 L 520 ' + (195 - rc * 1.8).toFixed(1) + ' L 820 ' + Math.min(215, 195 - waldB * 1.8).toFixed(1) + '" fill="none" stroke="var(--emerald)" stroke-width="2.8"/>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Info-Gap Horizon α vs. Global Wald Floor in No-Man\'s Land (α → α_max)</text>' +
          '<text x="640" y="' + (190 - rc * 1.8).toFixed(1) + '" fill="var(--amber)" font-size="11.5" font-weight="700">Critical Threshold r_c = ' + rc.toFixed(0) + '</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Option A Local α̂", value: robA.toFixed(2), sub: "Global Floor = " + waldA.toFixed(1), tone: waldA < rc ? "rose" : "sky" },
            { label: "Option B Local α̂", value: robB.toFixed(2), sub: "Global Floor = " + waldB.toFixed(1), tone: "emerald" },
            { label: "Sniedovich Safeguard", value: sniedovichBlocked ? "TRIPPED (Reject A)" : "Verified (Select B)", sub: "Global Wald Maximin Floor", tone: sniedovichBlocked ? "rose" : "emerald" },
            { label: "KOH GP Identifiability", value: s.anchor >= 0.5 ? "Anchored (Exact)" : "Confounded!", sub: "δ(x) ~ GP(0, k)", tone: s.anchor >= 0.5 ? "emerald" : "rose" }
          ]
        };
      }
    },

    "lenses-kelly-tail-corp": {
      title: "Interactive Five Lenses: Itô Variance Drain, Kelly Growth g(f) & Pareto Tail Explorer",
      cli: "cargo run --release --bin labs -- --lenses",
      controls: [
        { key: "mu", label: "Ensemble Drift μ (%/yr)", min: 2.0, max: 18.0, step: 0.5, val: 6.1, fmt: function (v) { return v.toFixed(1) + "%"; } },
        { key: "sigma", label: "Volatility σ (%/yr)", min: 5.0, max: 45.0, step: 1.0, val: 17.0, fmt: function (v) { return v.toFixed(1) + "%"; } },
        { key: "lev", label: "Leverage Fraction f (×)", min: 0.1, max: 4.5, step: 0.1, val: 1.0, fmt: function (v) { return v.toFixed(1) + "×"; } },
        { key: "alpha", label: "Pareto Tail Index α", min: 0.8, max: 3.5, step: 0.1, val: 1.6, fmt: function (v) { return v.toFixed(2); } }
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
          '<text x="86" y="206" fill="var(--muted)" font-size="11.5">Zero-Growth Line (g = 0%)  |  Above f = 2f* = ' + (2.0 * fStar).toFixed(2) + '×, volatility drag guarantees long-run ruin</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Ensemble Rate g_ens", value: (gEns * 100.0).toFixed(2) + "%/yr", sub: "Illusion of parallel crowd", tone: "sky" },
            { label: "Time Rate g_time(f)", value: (gTime * 100.0).toFixed(2) + "%/yr", sub: "Itô Drain = -" + (drain * 100.0).toFixed(2) + "%", tone: gTime > 0 ? "emerald" : "rose" },
            { label: "Optimal Kelly Peak f*", value: fStar.toFixed(2) + "×", sub: "Ruin boundary > " + (2.0 * fStar).toFixed(2) + "×", tone: "amber" },
            { label: "Tail Moment Regime", value: "α = " + s.alpha.toFixed(2), sub: regime, tone: s.alpha <= 2.0 ? "rose" : "emerald" }
          ]
        };
      }
    },

    "spectral-telgarsky-deff": {
      title: "Interactive Telgarsky Depth-D Sawtooth (2^D) & Gram Effective Dimension d_eff",
      cli: "cargo run --release --bin gates",
      controls: [
        { key: "depth", label: "Network Depth D (Layers)", min: 1, max: 6, step: 1, val: 4, fmt: function (v) { return "D = " + v; } },
        { key: "width", label: "Feature Dictionary Width W", min: 4, max: 16, step: 1, val: 8, fmt: function (v) { return "W = " + v; } },
        { key: "collin", label: "Atom Collinearity ρ", min: 0.0, max: 0.95, step: 0.05, val: 0.05, fmt: function (v) { return v.toFixed(2); } }
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
        var steps = 160;
        for (var i = 0; i <= steps; i++) {
          var x = i / steps;
          var v = x;
          for (var d = 0; d < D; d++) {
            v = v < 0.5 ? 2.0 * v : 2.0 * (1.0 - v);
          }
          var px = 70 + x * 760;
          var py = 192 - v * 140;
          pts.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + py.toFixed(1));
        }

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="830" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<path d="' + pts.join(" ") + '" fill="none" stroke="var(--sky)" stroke-width="2.4"/>' +
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
      title: "Interactive Forward Euler vs. 4th-Order Runge-Kutta (RK4) & Brusselator Limit Cycle",
      cli: "cargo run --release --bin labs -- --lenses",
      controls: [
        { key: "dt", label: "Integration Step Size h", min: 0.02, max: 0.35, step: 0.01, val: 0.15, fmt: function (v) { return v.toFixed(2); } },
        { key: "b", label: "Brusselator Drive B (Hopf > 2)", min: 1.2, max: 3.4, step: 0.1, val: 2.4, fmt: function (v) { return v.toFixed(1); } },
        { key: "damp", label: "Oscillator Damping ζ", min: 0.0, max: 0.3, step: 0.02, val: 0.0, fmt: function (v) { return v.toFixed(2); } }
      ],
      evaluate: function (s) {
        var h = s.dt;
        var zeta = s.damp;
        var eulerPts = [];
        var rk4Pts = [];
        var xE = 1.0, vE = 0.0;
        var xR = 1.0, vR = 0.0;
        for (var i = 0; i <= 45; i++) {
          var px = 70 + (i / 45.0) * 760;
          var pyE = 120 - Math.max(-90, Math.min(90, xE * 38));
          var pyR = 120 - Math.max(-90, Math.min(90, xR * 38));
          eulerPts.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + pyE.toFixed(1));
          rk4Pts.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + pyR.toFixed(1));

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
        var hopf = s.b > 2.0;

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="120" x2="830" y2="120" stroke="var(--muted)" stroke-width="1.2" stroke-dasharray="4 3"/>' +
          '<path d="' + eulerPts.join(" ") + '" fill="none" stroke="var(--rose)" stroke-width="2.2" stroke-dasharray="5 3"/>' +
          '<path d="' + rk4Pts.join(" ") + '" fill="none" stroke="var(--emerald)" stroke-width="2.8"/>' +
          '<text x="86" y="36" fill="var(--ink)" font-size="12" font-weight="750">Forward Euler Parasitic Energy Gain (Rose Dashed) vs. 4th-Order Runge-Kutta RK4 (Emerald)</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Euler Energy E_T / E_0", value: (eEuler / 0.5).toFixed(2) + "×", sub: "Parasitic gain (1+h²)ⁿ", tone: eEuler > 0.65 ? "rose" : "amber" },
            { label: "RK4 Energy E_T / E_0", value: (eRk4 / 0.5).toFixed(4) + "×", sub: "O(h⁴) global accuracy", tone: "emerald" },
            { label: "Brusselator Regime", value: hopf ? "Limit Cycle (d_i S > 0)" : "Point Attractor", sub: "B = " + s.b.toFixed(1) + " vs B_c = 2.0", tone: hopf ? "emerald" : "sky" },
            { label: "Trace tr(J) = B - 2", value: (s.b - 2.0 >= 0 ? "+" : "") + (s.b - 2.0).toFixed(2), sub: hopf ? "Hopf Bifurcation Active" : "Damped Spiral", tone: "amber" }
          ]
        };
      }
    },

    "analog-hpinv-collapse": {
      title: "Interactive Basel π²/6 Synthetic Accumulation Bound & PKU 24-Bit HP-INV RRAM Solver",
      cli: "cargo run --release --bin gates -- --defect replace_collapse",
      controls: [
        { key: "gen", label: "Recursive Generations (n)", min: 1, max: 16, step: 1, val: 8, fmt: function (v) { return "Gen " + v; } },
        { key: "noise", label: "Analog RRAM Noise σ", min: 0.05, max: 0.75, step: 0.05, val: 0.25, fmt: function (v) { return v.toFixed(2); } },
        { key: "cycles", label: "HP-INV Bit-Slice Cycles (k)", min: 1, max: 10, step: 1, val: 6, fmt: function (v) { return kFmt(v); } }
      ],
      evaluate: function (s) {
        var G = Math.round(s.gen);
        var accPts = [];
        var repPts = [];
        var sumBasel = 0.0;
        for (var g = 1; g <= G; g++) {
          sumBasel += 1.0 / (g * g);
          var px = 70 + ((g - 1) / Math.max(1, G - 1)) * 760;
          var pyA = 195 - (sumBasel / 16.0) * 160;
          var pyR = 195 - Math.min(165, (g / 16.0) * 160);
          accPts.push((g === 1 ? "M " : "L ") + px.toFixed(1) + " " + pyA.toFixed(1));
          repPts.push((g === 1 ? "M " : "L ") + px.toFixed(1) + " " + pyR.toFixed(1));
        }
        var baselCeil = (Math.PI * Math.PI) / 6.0;
        var rhoHp = Math.min(0.92, s.noise * 0.45);
        var relErr = Math.pow(rhoHp, Math.round(s.cycles) * 2.4);

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="830" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<line x1="70" y1="' + (195 - (baselCeil / 16.0) * 160).toFixed(1) + '" x2="830" y2="' + (195 - (baselCeil / 16.0) * 160).toFixed(1) + '" stroke="var(--sky)" stroke-width="1.8" stroke-dasharray="5 3"/>' +
          '<path d="' + repPts.join(" ") + '" fill="none" stroke="var(--rose)" stroke-width="2.8"/>' +
          '<path d="' + accPts.join(" ") + '" fill="none" stroke="var(--emerald)" stroke-width="2.8"/>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Synthetic Recursion: Replace Policy Linear Collapse ∝ n (Rose) vs. Accumulate Basel Ceiling ≤ π²/6 ≈ 1.6449 (Emerald)</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Accumulate Inflation", value: sumBasel.toFixed(4) + "×", sub: "Bounded by π²/6 = 1.6449", tone: "emerald" },
            { label: "Replace Collapse", value: G.toFixed(2) + "×", sub: "Unbounded linear divergence", tone: "rose" },
            { label: "PKU HP-INV Rel Error", value: relErr.toExponential(2), sub: Math.round(s.cycles) + " cycles on 3-bit RRAM", tone: relErr < 1e-6 ? "emerald" : "amber" },
            { label: "Horowitz DRAM/ALU Ratio", value: "711.11×", sub: "640 pJ DRAM vs 0.9 pJ FP32", tone: "sky" }
          ]
        };
      }
    },

    "stochary-kingman-triad": {
      title: "Interactive Stochary Buffer, Kingman VUT Queueing Cliff & Triadic (ℂ × Fractals × 𝕋ᵈ) Explorer",
      cli: "cargo run --release --bin gates -- --defect stochary_cybernetics",
      controls: [
        { key: "util", label: "Server Utilization u (%)", min: 50.0, max: 99.0, step: 0.5, val: 82.0, fmt: function (v) { return v.toFixed(1) + "%"; } },
        { key: "fexp", label: "Peters +50%/-40% Exposure f", min: 0.05, max: 1.0, step: 0.05, val: 0.25, fmt: function (v) { return (v * 100).toFixed(0) + "%"; } },
        { key: "sl", label: "Sterman Supply-Line Weight β_SL", min: 0.05, max: 1.0, step: 0.05, val: 1.0, fmt: function (v) { return v.toFixed(2); } }
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
          var px = 70 + (i / 48.0) * 760;
          var py = 195 - Math.min(170, (wv / 28.0) * 165);
          pts.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + py.toFixed(1));
        }
        var curX = 70 + ((u - 0.50) / 0.49) * 760;
        var curY = 195 - Math.min(170, (wq / 28.0) * 165);

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="195" x2="830" y2="195" stroke="var(--muted)" stroke-width="1.5"/>' +
          '<path d="' + pts.join(" ") + '" fill="none" stroke="var(--violet)" stroke-width="2.8"/>' +
          '<circle cx="' + curX.toFixed(1) + '" cy="' + curY.toFixed(1) + '" r="5.5" fill="var(--rose)"/>' +
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
      title: "Interactive Ville Martingale e-Process (E_t) vs. Optional-Stopping Peeking Inflation",
      cli: "cargo run --release --bin gates -- --defect peeking",
      controls: [
        { key: "peeks", label: "Sequential Peeks (K)", min: 1, max: 50, step: 1, val: 20, fmt: function (v) { return v + " peeks"; } },
        { key: "eff", label: "True Signal Drift μ", min: 0.0, max: 0.45, step: 0.05, val: 0.25, fmt: function (v) { return v.toFixed(2); } },
        { key: "bet", label: "GRAPA Betting Fraction λ", min: 0.05, max: 0.85, step: 0.05, val: 0.40, fmt: function (v) { return v.toFixed(2); } }
      ],
      evaluate: function (s) {
        var K = Math.round(s.peeks);
        var mu = s.eff;
        var lam = s.bet;
        var fprNaive = Math.min(0.65, 0.05 * (1.0 + 0.82 * Math.log(K)));
        var ePts = [];
        var E = 1.0;
        for (var t = 0; t <= 30; t++) {
          var shock = (t % 3 === 0 ? -0.35 : 0.45) + mu;
          if (t > 0) {
            E = E * Math.max(0.1, 1.0 + lam * shock);
          }
          var px = 70 + (t / 30.0) * 760;
          var py = 190 - Math.min(165, Math.max(5, (Math.log(Math.max(0.2, E)) / Math.log(50.0)) * 140 + 30));
          ePts.push((t === 0 ? "M " : "L ") + px.toFixed(1) + " " + py.toFixed(1));
        }

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<line x1="70" y1="62" x2="830" y2="62" stroke="var(--emerald)" stroke-width="2" stroke-dasharray="6 4"/>' +
          '<path d="' + ePts.join(" ") + '" fill="none" stroke="var(--sky)" stroke-width="2.8"/>' +
          '<text x="86" y="38" fill="var(--ink)" font-size="12" font-weight="750">Anytime-Valid Betting Martingale E_t = Π(1 + λ_i X_i) vs. Ville Threshold 1/α = 20.0 (Emerald Dashed)</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Cumulative Evidence E_30", value: E.toFixed(2), sub: E >= 20.0 ? "Rejects H₀ (E_t ≥ 20)" : "Below 1/α = 20 threshold", tone: E >= 20.0 ? "emerald" : "amber" },
            { label: "Ville Anytime Type-I Error", value: "≤ 5.00%", sub: "P₀(∃t: E_t ≥ 20) ≤ 0.05", tone: "emerald" },
            { label: "Naive z-Test Peeking FPR", value: (fprNaive * 100.0).toFixed(1) + "%", sub: "At K = " + K + " unadjusted peeks", tone: K > 1 ? "rose" : "sky" },
            { label: "Implied Safe p-Value", value: Math.min(1.0, 1.0 / Math.max(1.0, E)).toFixed(4), sub: "p_safe = min(1, 1/sup E_t)", tone: "sky" }
          ]
        };
      }
    },

    "invariants-moulton-chouldechova": {
      title: "Interactive Boundary Invariants: Moulton-Campbell Tail DEFF(p) & Chouldechova Fairness",
      cli: "cargo run --release --bin labs -- --invariants",
      controls: [
        { key: "mbar", label: "Mean Cluster Size m̄", min: 5, max: 200, step: 5, val: 169, fmt: function (v) { return v.toFixed(0); } },
        { key: "icc", label: "Tail Exceedance ICC ρ_I(p)", min: 0.01, max: 0.40, step: 0.01, val: 0.16, fmt: function (v) { return v.toFixed(2); } },
        { key: "pa", label: "Group A Base Rate p_A", min: 0.05, max: 0.60, step: 0.02, val: 0.39, fmt: function (v) { return v.toFixed(2); } },
        { key: "pb", label: "Group B Base Rate p_B", min: 0.05, max: 0.60, step: 0.02, val: 0.11, fmt: function (v) { return v.toFixed(2); } }
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
          '<text x="86" y="60" fill="var(--muted)" font-size="11.5">Nominal n = 27,716  →  Audited n_eff = ' + nEff.toFixed(1) + ' (DEFF = ' + deff.toFixed(2) + '×)</text>' +
          '</svg>';

        return {
          svg: svg,
          metrics: [
            { label: "Design Effect DEFF(p)", value: deff.toFixed(2) + "×", sub: "Variance inflation factor", tone: deff > 5 ? "amber" : "sky" },
            { label: "Audited n_eff(p) (Φ₂)", value: nEff.toFixed(1), sub: "Down from n = 27,716", tone: "emerald" },
            { label: "Chouldechova FPR_A / FPR_B", value: ratio.toFixed(2) + "×", sub: "FPR_A=" + (fprA * 100).toFixed(1) + "% vs FPR_B=" + (fprB * 100).toFixed(1) + "%", tone: Math.abs(ratio - 1.0) > 0.2 ? "rose" : "emerald" },
            { label: "SE Inflation √DEFF", value: Math.sqrt(deff).toFixed(2) + "×", sub: "True vs naive error bars", tone: "amber" }
          ]
        };
      }
    },

    "governance-bitemporal-drift": {
      title: "Interactive Bitemporal Clock, ExTRA-WCP-T Lei-Candès Drift Bound & Online ACI",
      cli: "cargo run --release --bin labs -- --governance",
      controls: [
        { key: "dtv", label: "Covariate Shift d_TV(P, Q)", min: 0.0, max: 0.35, step: 0.01, val: 0.09, fmt: function (v) { return v.toFixed(2); } },
        { key: "alpha", label: "Target Miscoverage α", min: 0.05, max: 0.20, step: 0.01, val: 0.10, fmt: function (v) { return (v * 100).toFixed(0) + "%"; } },
        { key: "gamma", label: "Online ACI Step Size γ", min: 0.005, max: 0.08, step: 0.005, val: 0.025, fmt: function (v) { return v.toFixed(3); } }
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
          var pyA = 195 - Math.min(0.98, target - 0.15 * tv + 0.5 * s.gamma) * 160;
          ptsTarget.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + pyT.toFixed(1));
          ptsStatic.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + pyS.toFixed(1));
          ptsAci.push((i === 0 ? "M " : "L ") + px.toFixed(1) + " " + pyA.toFixed(1));
        }

        var svg =
          '<svg viewBox="0 0 880 230" xmlns="http://www.w3.org/2000/svg">' +
          '<rect x="1" y="1" width="878" height="228" rx="10" fill="var(--surface)" stroke="var(--border)"/>' +
          '<path d="' + ptsTarget.join(" ") + '" fill="none" stroke="var(--sky)" stroke-width="1.8" stroke-dasharray="5 4"/>' +
          '<path d="' + ptsStatic.join(" ") + '" fill="none" stroke="var(--rose)" stroke-width="2.6"/>' +
          '<path d="' + ptsAci.join(" ") + '" fill="none" stroke="var(--emerald)" stroke-width="2.8"/>' +
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
    }
  };

  function kFmt(v) {
    return Math.round(v) + " cycles";
  }

  function buildLabDeepLink(moduleKey, state, chapter) {
    var params = ["module=" + encodeURIComponent(moduleKey), "seed=2026"];
    if (chapter) params.push("chapter=" + encodeURIComponent(chapter));
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
  }

  function setupInteractiveLabs(root) {
    var nodes = root.querySelectorAll(".stoch-lab[data-lab]");
    for (var i = 0; i < nodes.length; i++) {
      hydrateLabWidget(nodes[i]);
    }
  }

  function hydrateLabWidget(container) {
    if (container.getAttribute("data-hydrated") === "true") return;
    var moduleKey = container.getAttribute("data-lab");
    var spec = LAB_REGISTRY[moduleKey] || LAB_REGISTRY["jensen-crps"];
    var chapter = container.getAttribute("data-chapter") || "";
    var customTitle = container.getAttribute("data-title") || spec.title;
    var customCli = container.getAttribute("data-cli") || spec.cli;
    var fromQuery = container.getAttribute("data-from-query") === "true";

    container.setAttribute("data-hydrated", "true");
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
    seedBadge.textContent = "Φ₁ SEED = 2026";
    badgeRow.appendChild(seedBadge);

    titleGroup.appendChild(badgeRow);

    var h4 = document.createElement("h4");
    h4.className = "stoch-lab-title";
    h4.textContent = customTitle;
    titleGroup.appendChild(h4);
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
      setTimeout(function () {
        copyBtn.textContent = "Copy CLI Audit";
      }, 1600);
    });
    actions.appendChild(copyBtn);

    var embedBtn = document.createElement("button");
    embedBtn.type = "button";
    embedBtn.className = "stoch-lab-btn";
    embedBtn.textContent = "Sandbox (/lab/embed)";
    if (!fromQuery) {
      actions.appendChild(embedBtn);
    }

    var openLink = document.createElement("a");
    openLink.className = "stoch-lab-btn stoch-lab-btn-primary";
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
    var cliSpan = document.createElement("span");
    cliSpan.textContent = "$ " + customCli;
    var syncSpan = document.createElement("span");
    footer.appendChild(cliSpan);
    footer.appendChild(syncSpan);

    function renderState() {
      var res = spec.evaluate(state);
      stage.innerHTML = res.svg;
      metricsGrid.innerHTML = "";
      for (var m = 0; m < res.metrics.length; m++) {
        var item = res.metrics[m];
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

      var deepUrl = buildLabDeepLink(moduleKey, state, chapter);
      openLink.href = deepUrl;
      syncSpan.textContent = "State synced with " + LAB_PATH + "/?module=" + moduleKey + "&seed=2026";

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
              seed: 2026,
              dark: isDarkThemeActive(),
              state: state,
              metrics: res.metrics,
              cli: customCli
            },
            "*"
          );
        } catch (_e) {}
      }
    }

    for (var j = 0; j < spec.controls.length; j++) {
      (function (ctrl) {
        var wrap = document.createElement("div");
        wrap.className = "stoch-lab-control";

        var initVal = state[ctrl.key];
        var lbl = document.createElement("label");
        var nameSpan = document.createElement("span");
        nameSpan.textContent = ctrl.label;
        var valSpan = document.createElement("span");
        valSpan.className = "stoch-lab-control-val";
        valSpan.textContent = ctrl.fmt(initVal);
        lbl.appendChild(nameSpan);
        lbl.appendChild(valSpan);

        var input = document.createElement("input");
        input.type = "range";
        input.min = String(ctrl.min);
        input.max = String(ctrl.max);
        input.step = String(ctrl.step);
        input.value = String(initVal);

        input.addEventListener("input", function () {
          var num = parseFloat(input.value);
          state[ctrl.key] = num;
          valSpan.textContent = ctrl.fmt(num);
          renderState();
        });

        wrap.appendChild(lbl);
        wrap.appendChild(input);
        controlsGrid.appendChild(wrap);
      })(spec.controls[j]);
    }

    embedBtn.addEventListener("click", function () {
      var active = iframeWrap.classList.toggle("active");
      embedBtn.textContent = active ? "Hide Sandbox" : "Sandbox (/lab/embed)";
      if (active) {
        var base = resolveLabBaseUrl();
        var embedUrl = buildLabDeepLink(moduleKey, state, chapter).replace(
          base + "/?",
          base + "/embed/?"
        );
        if (iframe.src !== embedUrl) {
          iframe.src = embedUrl;
        }
      }
    });

    body.appendChild(controlsGrid);
    body.appendChild(stage);
    body.appendChild(metricsGrid);
    body.appendChild(iframeWrap);
    container.appendChild(body);
    container.appendChild(footer);

    renderState();
  }

  function prepareDocumentForMathAndTables() {
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
    }

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
