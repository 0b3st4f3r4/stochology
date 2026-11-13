(function () {
  "use strict";

  var REPO_URL = "https://github.com/0b3st4f3r4/stochology";
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

  

  function normalizeMathText(text) {
    if (text.indexOf("$") === -1) return text;
    return text.replace(/(\$\$[\s\S]*?\$\$|\$[^$\n]+\$)/g, function (seg) {
      return seg
        .replace(/’/g, "'")
        .replace(/(^|[^\\])%/g, "$1\\%");
    });
  }

    function createSplitMix64(seedNum) {
    var s0 = Number(seedNum);
    if (!isFinite(s0) || s0 <= 0) s0 = 2026;
    if (typeof BigInt === "function") {
      var GOLDEN = BigInt("0x9E3779B97F4A7C15");
      var MIX1 = BigInt("0xBF58476D1CE4E5B9");
      var MIX2 = BigInt("0x94D049BB133111EB");
      var C1 = BigInt("0xD1342543DE82EF95");
      var C2 = BigInt("0xA5A5A5A55A5A5A5A");
      var MASK = BigInt("0xFFFFFFFFFFFFFFFF");

      function mix64(z) {
        z = ((z ^ (z >> BigInt(30))) * MIX1) & MASK;
        z = ((z ^ (z >> BigInt(27))) * MIX2) & MASK;
        return (z ^ (z >> BigInt(31))) & MASK;
      }

      function makeFromRaw(initState, initGamma) {
        var state = initState & MASK;
        var gamma = (initGamma & MASK) | BigInt(1);
        return {
          nextU64: function () {
            state = (state + gamma) & MASK;
            return mix64(state);
          },
          nextHex: function () {
            var u = this.nextU64();
            var hex = u.toString(16);
            while (hex.length < 16) hex = "0" + hex;
            return "0x" + hex;
          },
          nextF64: function () {
            var u = this.nextU64() >> BigInt(11);
            return Number(u) * Math.pow(2, -53);
          },
          split: function (streamId) {
            var sid = BigInt(Math.floor(Number(streamId) || 0)) & MASK;
            var base = this.nextU64();
            var mixedId = mix64((sid + GOLDEN) & MASK);
            var childGamma = (((sid * C1) & MASK) ^ C2) | BigInt(1);
            return makeFromRaw((base ^ mixedId) & MASK, childGamma);
          }
        };
      }

      var initialState = BigInt(Math.floor(s0)) & MASK;
      return makeFromRaw(initialState, GOLDEN);
    }
    var x = Math.floor(s0) % 2147483647;
    if (x <= 0) x += 2147483646;
    return {
      nextU64: function () {
        x = (x * 16807) % 2147483647;
        return x;
      },
      nextHex: function () {
        var u = this.nextU64();
        return "0x" + u.toString(16);
      },
      nextF64: function () {
        x = (x * 16807) % 2147483647;
        return (x - 1) / 2147483646;
      },
      split: function (streamId) {
        return createSplitMix64(s0 + (Number(streamId) || 1) * 104729);
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

  

  function injectTopMenuLogo() {
    var menuTitle = document.querySelector("#mdbook-menu-bar .menu-title");
    if (!menuTitle || menuTitle.querySelector(".stoch-topbar-emblem")) return;

    menuTitle.setAttribute("aria-label", "Stochology — Scroll to top");
    menuTitle.setAttribute("title", "Stochology — Scroll to top");

    var svg = [
      '<span class="visually-hidden">STOCHOLOGY</span>',
      '<svg class="stoch-topbar-emblem" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 128" fill="none" role="img" aria-hidden="true">',
      '<defs>',
      '<radialGradient id="topbar-glow" cx="52%" cy="52%" r="50%">',
      '<stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>',
      '<stop offset="48%" stop-color="#38bdf8" stop-opacity="0.14"/>',
      '<stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>',
      '<stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>',
      '</radialGradient>',
      '<linearGradient id="topbar-bridge-l1-l2" x1="23" y1="74" x2="51" y2="26" gradientUnits="userSpaceOnUse">',
      '<stop offset="0%" stop-color="#059669"/>',
      '<stop offset="55%" stop-color="var(--fav-green)"/>',
      '<stop offset="100%" stop-color="var(--fav-blue)"/>',
      '</linearGradient>',
      '<linearGradient id="topbar-bridge-l2-l3" x1="78" y1="26" x2="105" y2="75" gradientUnits="userSpaceOnUse">',
      '<stop offset="0%" stop-color="var(--fav-blue)"/>',
      '<stop offset="52%" stop-color="var(--fav-gold)"/>',
      '<stop offset="100%" stop-color="var(--fav-crimson)"/>',
      '</linearGradient>',
      '<linearGradient id="topbar-bridge-l3-l1" x1="91" y1="98" x2="36" y2="98" gradientUnits="userSpaceOnUse">',
      '<stop offset="0%" stop-color="var(--fav-crimson)"/>',
      '<stop offset="50%" stop-color="var(--fav-gold)"/>',
      '<stop offset="100%" stop-color="var(--fav-green)"/>',
      '</linearGradient>',
      '<linearGradient id="topbar-prism-grad" x1="64" y1="24" x2="64" y2="87" gradientUnits="userSpaceOnUse">',
      '<stop offset="0%" stop-color="var(--fav-blue)" stop-opacity="0.75"/>',
      '<stop offset="50%" stop-color="var(--fav-gold)" stop-opacity="0.85"/>',
      '<stop offset="100%" stop-color="var(--fav-frame)" stop-opacity="0.75"/>',
      '</linearGradient>',
      '<linearGradient id="topbar-tail-fill" x1="31" y1="86" x2="96" y2="48" gradientUnits="userSpaceOnUse">',
      '<stop offset="0%" stop-color="#059669" stop-opacity="0.46"/>',
      '<stop offset="38%" stop-color="#2563eb" stop-opacity="0.48"/>',
      '<stop offset="68%" stop-color="#f59e0b" stop-opacity="0.44"/>',
      '<stop offset="100%" stop-color="#e11d48" stop-opacity="0.48"/>',
      '</linearGradient>',
      '<linearGradient id="topbar-curve-stroke" x1="31" y1="85" x2="96" y2="84" gradientUnits="userSpaceOnUse">',
      '<stop offset="0%" stop-color="var(--fav-ink)"/>',
      '<stop offset="45%" stop-color="var(--fav-eyelid-mid)"/>',
      '<stop offset="100%" stop-color="var(--fav-ink)"/>',
      '</linearGradient>',
      '<radialGradient id="topbar-pupil-abyss" cx="45%" cy="42%" r="58%">',
      '<stop offset="0%" stop-color="#020617"/>',
      '<stop offset="68%" stop-color="#0f172a"/>',
      '<stop offset="100%" stop-color="#451a03"/>',
      '</radialGradient>',
      '<linearGradient id="topbar-gold-mira" x1="58" y1="52" x2="84" y2="82" gradientUnits="userSpaceOnUse">',
      '<stop offset="0%" stop-color="#fef08a"/>',
      '<stop offset="45%" stop-color="#fbbf24"/>',
      '<stop offset="100%" stop-color="#d97706"/>',
      '</linearGradient>',
      '<linearGradient id="topbar-gold-axes" x1="41" y1="66.65" x2="72" y2="40" gradientUnits="userSpaceOnUse">',
      '<stop offset="0%" stop-color="#d97706"/>',
      '<stop offset="50%" stop-color="#fbbf24"/>',
      '<stop offset="100%" stop-color="#fef08a"/>',
      '</linearGradient>',
      '<radialGradient id="topbar-gold-core" cx="38%" cy="35%" r="65%">',
      '<stop offset="0%" stop-color="#ffffff"/>',
      '<stop offset="35%" stop-color="#fde047"/>',
      '<stop offset="100%" stop-color="#d97706"/>',
      '</radialGradient>',
      '<radialGradient id="topbar-node-well" cx="38%" cy="35%" r="65%">',
      '<stop offset="0%" stop-color="var(--fav-well-hi)"/>',
      '<stop offset="100%" stop-color="var(--fav-node-bg)"/>',
      '</radialGradient>',
      '<linearGradient id="topbar-ring-l2" x1="55" y1="15" x2="73" y2="33" gradientUnits="userSpaceOnUse">',
      '<stop offset="0%" stop-color="#7dd3fc"/>',
      '<stop offset="50%" stop-color="#2563eb"/>',
      '<stop offset="100%" stop-color="#1e3a8a"/>',
      '</linearGradient>',
      '<radialGradient id="topbar-core-l2" cx="35%" cy="35%" r="65%">',
      '<stop offset="0%" stop-color="#bae6fd"/>',
      '<stop offset="45%" stop-color="#3b82f6"/>',
      '<stop offset="100%" stop-color="#1d4ed8"/>',
      '</radialGradient>',
      '<linearGradient id="topbar-ring-l1" x1="18" y1="78" x2="37" y2="96" gradientUnits="userSpaceOnUse">',
      '<stop offset="0%" stop-color="#6ee7b7"/>',
      '<stop offset="50%" stop-color="#10b981"/>',
      '<stop offset="100%" stop-color="#065f46"/>',
      '</linearGradient>',
      '<radialGradient id="topbar-core-l1" cx="35%" cy="35%" r="65%">',
      '<stop offset="0%" stop-color="#a7f3d0"/>',
      '<stop offset="45%" stop-color="#10b981"/>',
      '<stop offset="100%" stop-color="#047857"/>',
      '</radialGradient>',
      '<linearGradient id="topbar-ring-l3" x1="91" y1="78" x2="110" y2="96" gradientUnits="userSpaceOnUse">',
      '<stop offset="0%" stop-color="#fda4af"/>',
      '<stop offset="50%" stop-color="#e11d48"/>',
      '<stop offset="100%" stop-color="#881337"/>',
      '</linearGradient>',
      '<radialGradient id="topbar-core-l3" cx="35%" cy="35%" r="65%">',
      '<stop offset="0%" stop-color="#fecdd3"/>',
      '<stop offset="45%" stop-color="#f43f5e"/>',
      '<stop offset="100%" stop-color="#be123c"/>',
      '</radialGradient>',
      '</defs>',
      '<circle cx="64" cy="66" r="48" fill="url(#topbar-glow)"/>',
      '<path d="M 22.77 74.01 A 42 42 0 0 1 50.60 26.19" stroke="url(#topbar-bridge-l1-l2)" stroke-width="4.4" stroke-linecap="round"/>',
      '<path d="M 46.92 32.93 L 51.02 26.06 L 43.36 23.74" stroke="url(#topbar-bridge-l1-l2)" stroke-width="4.0" stroke-linecap="round" stroke-linejoin="round"/>',
      '<path d="M 77.67 26.29 A 42 42 0 0 1 105.17 74.30" stroke="url(#topbar-bridge-l2-l3)" stroke-width="4.4" stroke-linecap="round"/>',
      '<path d="M 101.18 67.75 L 105.08 74.73 L 110.91 69.26" stroke="url(#topbar-bridge-l2-l3)" stroke-width="4.0" stroke-linecap="round" stroke-linejoin="round"/>',
      '<path d="M 91.55 97.70 A 42 42 0 0 1 36.22 97.50" stroke="url(#topbar-bridge-l3-l1)" stroke-width="4.4" stroke-linecap="round"/>',
      '<path d="M 43.90 97.32 L 35.90 97.21 L 37.72 105.00" stroke="url(#topbar-bridge-l3-l1)" stroke-width="4.0" stroke-linecap="round" stroke-linejoin="round"/>',
      '<polygon points="64,24 100.37,87 27.63,87" stroke="url(#topbar-prism-grad)" stroke-width="2.4" stroke-dasharray="3.64 3.64" stroke-linejoin="round"/>',
      '<path d="M 31 85 C 40 85, 45 46, 54 46 C 63 46, 68 74, 96 84 L 96 86 L 31 86 Z" fill="url(#topbar-tail-fill)"/>',
      '<path d="M 31 85 C 40 85, 45 46, 54 46 C 63 46, 68 74, 96 84" stroke="url(#topbar-curve-stroke)" stroke-width="2.6" stroke-linecap="round"/>',
      '<line x1="44.3" y1="66.65" x2="59.5" y2="66.65" stroke="url(#topbar-gold-axes)" stroke-width="2.6" stroke-dasharray="3.2 2.8" stroke-linecap="round"/>',
      '<line x1="72" y1="43.5" x2="72" y2="53.5" stroke="url(#topbar-gold-axes)" stroke-width="2.6" stroke-dasharray="3.4 3.2" stroke-linecap="round"/>',
      '<line x1="72" y1="80.0" x2="72" y2="83.4" stroke="url(#topbar-gold-axes)" stroke-width="2.6" stroke-linecap="round"/>',
      '<g id="topbar-golden-reticle">',
      '<circle cx="72" cy="66.65" r="7.4" fill="var(--fav-node-bg)"/>',
      '<circle cx="72" cy="66.65" r="6.4" fill="url(#topbar-pupil-abyss)" stroke="url(#topbar-gold-mira)" stroke-width="2.4"/>',
      '<line x1="62.4" y1="66.65" x2="67.8" y2="66.65" stroke="url(#topbar-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>',
      '<line x1="76.2" y1="66.65" x2="81.6" y2="66.65" stroke="url(#topbar-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>',
      '<line x1="72" y1="57.05" x2="72" y2="62.45" stroke="url(#topbar-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>',
      '<line x1="72" y1="70.85" x2="72" y2="76.25" stroke="url(#topbar-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>',
      '<circle cx="72" cy="66.65" r="2.2" fill="url(#topbar-gold-core)"/>',
      '</g>',
      '<circle cx="64" cy="24" r="9.2" fill="url(#topbar-node-well)" stroke="url(#topbar-ring-l2)" stroke-width="3.4"/>',
      '<circle cx="64" cy="24" r="4.0" fill="url(#topbar-core-l2)"/>',
      '<circle cx="27.63" cy="87" r="9.2" fill="url(#topbar-node-well)" stroke="url(#topbar-ring-l1)" stroke-width="3.4"/>',
      '<circle cx="27.63" cy="87" r="4.0" fill="url(#topbar-core-l1)"/>',
      '<circle cx="100.37" cy="87" r="9.2" fill="url(#topbar-node-well)" stroke="url(#topbar-ring-l3)" stroke-width="3.4"/>',
      '<circle cx="100.37" cy="87" r="4.0" fill="url(#topbar-core-l3)"/>',
      '</svg>'
    ].join("");

    menuTitle.innerHTML = svg;
  }

  function dismissPageLoader() {
    var loader = document.getElementById("stoch-page-loader");
    if (!loader || loader.getAttribute("data-dismissed") === "true") return;
    loader.setAttribute("data-dismissed", "true");

    setTimeout(function () {
      loader.classList.add("stoch-loader-hidden");
      setTimeout(function () {
        if (loader && loader.parentNode) {
          loader.parentNode.removeChild(loader);
        }
      }, 400);
    }, 150);
  }

  ;

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

  

  if (typeof window !== "undefined") {
    window.createStochologySplitMix64 = createSplitMix64;
  }

  

  function syncSliderCssVars(inputEl, minVal, maxVal, curVal) {
    inputEl.style.setProperty("--attr-min", String(minVal));
    inputEl.style.setProperty("--attr-max", String(maxVal));
    inputEl.style.setProperty("--control-value", String(curVal));
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
        var label = prev ? prev.textContent.trim() : "Specification table";
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

  var svgZoomDialogState = null;

  function ensureSvgZoomDialog() {
    if (svgZoomDialogState) return svgZoomDialogState;

    var dialog = document.createElement("dialog");
    dialog.id = "stoch-svg-zoom-dialog";
    dialog.className = "stoch-svg-zoom-dialog";
    dialog.setAttribute("closedby", "any");
    dialog.setAttribute("aria-labelledby", "stoch-svg-zoom-title");

    var header = document.createElement("div");
    header.className = "stoch-svg-zoom-header";

    var titleEl = document.createElement("h3");
    titleEl.id = "stoch-svg-zoom-title";
    titleEl.className = "stoch-svg-zoom-title";
    titleEl.textContent = "SVG Architectural Diagram";
    header.appendChild(titleEl);

    var controls = document.createElement("div");
    controls.className = "stoch-svg-zoom-controls";

    var btnOut = document.createElement("button");
    btnOut.type = "button";
    btnOut.className = "stoch-svg-zoom-btn";
    btnOut.textContent = "−";
    btnOut.setAttribute("aria-label", "Zoom out SVG");

    var btnReset = document.createElement("button");
    btnReset.type = "button";
    btnReset.className = "stoch-svg-zoom-btn";
    btnReset.textContent = "100%";
    btnReset.setAttribute("aria-label", "Reset SVG zoom to 100%");

    var btnIn = document.createElement("button");
    btnIn.type = "button";
    btnIn.className = "stoch-svg-zoom-btn";
    btnIn.textContent = "+";
    btnIn.setAttribute("aria-label", "Zoom in SVG");

    var btnClose = document.createElement("button");
    btnClose.type = "button";
    btnClose.className = "stoch-svg-zoom-btn stoch-svg-zoom-close-btn";
    btnClose.textContent = "✕ Close";
    btnClose.setAttribute("aria-label", "Close SVG zoom modal");

    controls.appendChild(btnOut);
    controls.appendChild(btnReset);
    controls.appendChild(btnIn);
    controls.appendChild(btnClose);
    header.appendChild(controls);

    var viewport = document.createElement("div");
    viewport.className = "stoch-svg-zoom-viewport";
    var stage = document.createElement("div");
    stage.className = "stoch-svg-zoom-stage";
    viewport.appendChild(stage);

    var captionEl = document.createElement("div");
    captionEl.className = "stoch-svg-zoom-caption";

    dialog.appendChild(header);
    dialog.appendChild(viewport);
    dialog.appendChild(captionEl);
    document.body.appendChild(dialog);

    var zoomPct = 100;
    var baseWidthPx = 0;

    function computeBaseWidth() {
      var vw = viewport.clientWidth;
      if (vw && vw > 60) {
        return vw - 44;
      }
      return Math.max(320, Math.min(window.innerWidth || 960, 1340) - 80);
    }

    function setZoomPct(nextPct) {
      zoomPct = Math.max(50, Math.min(300, nextPct));
      if (!baseWidthPx || baseWidthPx < 100) {
        baseWidthPx = computeBaseWidth();
      }
      var targetPx = Math.round((baseWidthPx * zoomPct) / 100);
      stage.style.width = targetPx + "px";
      stage.style.inlineSize = targetPx + "px";
      var child = stage.firstElementChild;
      if (child && child.style) {
        child.style.width = targetPx + "px";
        child.style.inlineSize = targetPx + "px";
        child.style.maxWidth = "none";
        child.style.maxInlineSize = "none";
        child.style.height = "auto";
        child.style.blockSize = "auto";
      }
      btnReset.textContent = zoomPct + "%";
    }

    function closeDialog() {
      if (typeof dialog.close === "function") {
        try {
          if (dialog.open) {
            dialog.close();
          }
        } catch (_) {}
      }
      dialog.removeAttribute("open");
    }

    btnOut.addEventListener("click", function (event) {
      event.preventDefault();
      event.stopPropagation();
      setZoomPct(zoomPct - 25);
    });
    btnIn.addEventListener("click", function (event) {
      event.preventDefault();
      event.stopPropagation();
      setZoomPct(zoomPct + 25);
    });
    btnReset.addEventListener("click", function (event) {
      event.preventDefault();
      event.stopPropagation();
      baseWidthPx = computeBaseWidth();
      setZoomPct(100);
    });
    btnClose.addEventListener("click", function (event) {
      event.preventDefault();
      event.stopPropagation();
      closeDialog();
    });

    dialog.addEventListener("click", function (event) {
      if (event.target === dialog) {
        closeDialog();
      }
    });

    document.addEventListener(
      "keydown",
      function (event) {
        if (event.key === "Escape" && (dialog.open || dialog.hasAttribute("open"))) {
          event.preventDefault();
          event.stopPropagation();
          closeDialog();
        }
      },
      true
    );

    svgZoomDialogState = {
      dialog: dialog,
      titleEl: titleEl,
      viewport: viewport,
      stage: stage,
      captionEl: captionEl,
      resetBaseWidth: function () {
        baseWidthPx = computeBaseWidth();
      },
      setZoomPct: setZoomPct,
      closeDialog: closeDialog
    };
    return svgZoomDialogState;
  }

  function openSvgZoomModal(targetEl, titleText, captionText) {
    if (!targetEl) return;
    var modal = ensureSvgZoomDialog();
    modal.titleEl.textContent = titleText || "SVG Architectural Diagram";
    modal.captionEl.textContent = captionText || "";
    modal.stage.innerHTML = "";

    var clone = targetEl.cloneNode(true);
    clone.removeAttribute("width");
    clone.removeAttribute("height");
    modal.stage.appendChild(clone);

    if (typeof modal.dialog.showModal === "function") {
      if (!modal.dialog.open) {
        try {
          modal.dialog.showModal();
        } catch (_) {
          modal.dialog.setAttribute("open", "open");
        }
      }
    } else {
      modal.dialog.setAttribute("open", "open");
    }

    modal.resetBaseWidth();
    modal.setZoomPct(100);
  }

  function setupSvgZoomModals(root) {
    var figures = root.querySelectorAll("figure.stoch-figure:not(.stoch-logo-figure)");
    for (var i = 0; i < figures.length; i++) {
      (function (fig) {
        if (fig.getAttribute("data-svg-zoom-ready") === "true") return;
        fig.setAttribute("data-svg-zoom-ready", "true");

        function resolveFigureTargetAndMeta() {
          var target =
            fig.querySelector("svg") ||
            fig.querySelector('img[src$=".svg"]');
          var cap = fig.querySelector("figcaption");
          var strong = cap ? cap.querySelector("strong") : null;
          var titleText = strong
            ? strong.textContent.trim()
            : fig.getAttribute("aria-label") ||
              (target && target.getAttribute("aria-label")) ||
              "SVG Architectural Diagram";
          var captionText = cap ? cap.textContent.trim() : "";
          return { target: target, titleText: titleText, captionText: captionText };
        }

        var actionsBar = document.createElement("div");
        actionsBar.className = "stoch-figure-actions";

        var zoomBtn = document.createElement("button");
        zoomBtn.type = "button";
        zoomBtn.className = "stoch-svg-zoom-trigger";
        zoomBtn.textContent = "⤢ Zoom SVG";
        zoomBtn.setAttribute("aria-label", "Open SVG diagram in full-screen zoom modal");
        zoomBtn.addEventListener("click", function () {
          var info = resolveFigureTargetAndMeta();
          if (info.target) {
            openSvgZoomModal(info.target, info.titleText, info.captionText);
          }
        });
        actionsBar.appendChild(zoomBtn);
        fig.insertBefore(actionsBar, fig.firstChild);

        fig.addEventListener("click", function (ev) {
          if (ev.target.closest("button, a, input, select, textarea, figcaption")) {
            return;
          }
          if (ev.target.closest("svg") || ev.target.closest('img[src$=".svg"]')) {
            var info = resolveFigureTargetAndMeta();
            if (info.target) {
              openSvgZoomModal(info.target, info.titleText, info.captionText);
            }
          }
        });
      })(figures[i]);
    }
  }

  // A link climbing one level above the site root points at a repository file outside the book
  // (crates/, data/, AGENTS.md, README.md). Send it to GitHub; mdBook renamed ".md" to ".html".
  function rewriteRepositoryLinks(root) {
    var depth = (getPathToRoot().match(/\.\.\//g) || []).length;
    var links = root.querySelectorAll('a[href^="../"]');
    for (var i = 0; i < links.length; i++) {
      var m = links[i].getAttribute("href").match(/^((?:\.\.\/)+)([^#]*)(#.*)?$/);
      if (!m || m[1].length / 3 !== depth + 1) continue;
      var path = m[2].replace(/\.html$/, ".md");
      var kind = path === "" || path.charAt(path.length - 1) === "/" ? "tree" : "blob";
      links[i].setAttribute("href", REPO_URL + "/" + kind + "/main/" + path + (m[3] || ""));
    }
  }

  // mdBook ships five fixed theme slots (light, rust, coal, navy, ayu). theme/book.css
  // re-skins each slot with a Stochology palette; this renames the visible list so the
  // menu speaks palette names while the slots keep mdBook's own persistence and
  // highlight-stylesheet mapping intact.
  var PALETTE_LABELS = {
    default_theme: "Auto",
    light: "Marble",
    rust: "Parchment",
    coal: "Graphite",
    navy: "Ultramarine",
    ayu: "Nocturne"
  };

  // Native menu plumbing. Both menus are popovers, which puts them in the top layer (the
  // stacking order costs no z-index), hands light dismissal and Escape to the browser, and
  // lets theme/book.css place each one under its own toggle with CSS anchor positioning.
  // A browser without the popover API keeps the imperative `display` toggle, and a browser
  // with popover but without anchor positioning keeps the measured offset from
  // pinMenuUnderButton, so nothing regresses where the primitives are missing.
  var MENU_HAS_POPOVER =
    typeof window.HTMLElement === "function" &&
    typeof HTMLElement.prototype.showPopover === "function" &&
    typeof HTMLElement.prototype.hidePopover === "function";
  var MENU_HAS_ANCHOR =
    typeof window.CSS === "object" &&
    typeof CSS.supports === "function" &&
    CSS.supports("anchor-name: --stoch-menu");

  function isMenuOpen(menu) {
    if (!menu) return false;
    return MENU_HAS_POPOVER ? menu.matches(":popover-open") : menu.style.display === "block";
  }

  function showMenu(menu) {
    if (!menu) return;
    menu.style.display = "";
    if (!MENU_HAS_POPOVER) {
      menu.style.display = "block";
      return;
    }
    if (isMenuOpen(menu)) return;
    try {
      menu.showPopover();
    } catch (e) {
      menu.style.display = "block";
    }
  }

  function hideMenu(menu) {
    if (!menu) return;
    if (!MENU_HAS_POPOVER) {
      menu.style.display = "none";
      return;
    }
    if (!isMenuOpen(menu)) return;
    try {
      menu.hidePopover();
    } catch (e) {
      menu.style.display = "none";
    }
  }

  // Keep the measured fallback alive only where anchor positioning is missing; elsewhere
  // the toggle and the menu are two anchor-positioned boxes and nothing is written inline.
  function positionMenu(button, menu) {
    if (MENU_HAS_ANCHOR) return;
    pinMenuUnderButton(button, menu);
  }

  // Wire one toggle/menu pair to the native path. Returns false when the popover API is
  // missing, so the caller can install the imperative fallback unchanged.
  function setupNativeMenu(toggle, menu, onOpen) {
    if (!toggle || !menu || !MENU_HAS_POPOVER) return false;
    menu.setAttribute("popover", "auto");
    if (toggle.tagName === "BUTTON") {
      // mdBook's own palette button can carry the declarative trigger, so it keeps it.
      toggle.setAttribute("popovertarget", menu.id);
      toggle.setAttribute("popovertargetaction", "toggle");
    } else {
      // The typeface control is an <a role="button">, like the rest of the icon row, and
      // `popovertarget` does not exist on anchors. `showPopover()` keeps everything the
      // declarative form gives (top layer, light dismiss, close watcher); only the toggle
      // semantics are written by hand. The pointerdown flag is what makes that work: the
      // pointerdown of a second click light-dismisses the open menu before the click
      // arrives, so the click must not reopen what that dismissal just closed.
      var dismissedAtPointerDown = false;
      toggle.addEventListener("pointerdown", function () {
        dismissedAtPointerDown = isMenuOpen(menu);
      });
      toggle.addEventListener("click", function (event) {
        event.preventDefault();
        if (dismissedAtPointerDown) {
          dismissedAtPointerDown = false;
          return;
        }
        if (isMenuOpen(menu)) hideMenu(menu);
        else showMenu(menu);
      });
      // Enter activates a link and lands in the click handler above; Space does not, so a
      // role=button anchor needs it spelled out.
      toggle.addEventListener("keydown", function (event) {
        if (event.key !== " " && event.key !== "Spacebar") return;
        event.preventDefault();
        if (isMenuOpen(menu)) hideMenu(menu);
        else showMenu(menu);
      });
    }
    menu.addEventListener("beforetoggle", function (event) {
      // mdBook's own close path writes `display: none`; it must not survive a reopen.
      if (event.newState === "open") menu.style.display = "";
    });
    menu.addEventListener("toggle", function (event) {
      var open = event.newState === "open";
      toggle.setAttribute("aria-expanded", open ? "true" : "false");
      if (!open) return;
      if (onOpen) onOpen();
      positionMenu(toggle, menu);
    });
    if (!MENU_HAS_ANCHOR) {
      window.addEventListener("resize", function () {
        positionMenu(toggle, menu);
      });
    }
    // mdBook's searcher calls preventDefault on every Escape keydown, and that cancels the
    // popover close watcher in the same event. Close the menu first, in the capture phase,
    // so Escape keeps meaning "close the topmost layer" and never leaks into the search.
    document.addEventListener(
      "keydown",
      function (event) {
        if (event.key !== "Escape" || !isMenuOpen(menu)) return;
        event.preventDefault();
        event.stopPropagation();
        hideMenu(menu);
        toggle.focus();
      },
      true
    );
    return true;
  }

  // mdBook persists the palette slot under `mdbook-theme`; no key means the Auto slot.
  function paletteActiveSlot() {
    var saved = null;
    try {
      saved = localStorage.getItem("mdbook-theme");
    } catch (e) {
      saved = null;
    }
    return saved || "default_theme";
  }

  function syncPaletteChecks(list) {
    var active = paletteActiveSlot();
    var buttons = list.querySelectorAll("button.theme");
    for (var i = 0; i < buttons.length; i++) {
      var slot = buttons[i].id.replace(/^mdbook-theme-/, "");
      buttons[i].setAttribute("aria-checked", slot === active ? "true" : "false");
    }
  }

  function initThemeLabels() {
    var list = document.getElementById("mdbook-theme-list");
    if (!list) return;
    Object.keys(PALETTE_LABELS).forEach(function (slot) {
      var button = list.querySelector("#mdbook-theme-" + slot);
      if (button) {
        button.textContent = PALETTE_LABELS[slot];
        button.setAttribute("role", "menuitemradio");
        button.setAttribute("aria-checked", "false");
      }
    });
    var toggle = document.getElementById("mdbook-theme-toggle");
    if (!toggle) return;
    toggle.title = "Change palette";
    toggle.setAttribute("aria-label", "Change palette");
    var nativeMenu = setupNativeMenu(toggle, list, function () {
      syncPaletteChecks(list);
      // mdBook's own show path focuses the active slot; keep that for arrow navigation.
      var selected = list.querySelector(".theme-selected");
      if (selected) selected.focus();
    });
    syncPaletteChecks(list);
    // Selecting a slot closes the menu in both paths; mdBook's own module leaves it open.
    list.addEventListener("click", function (event) {
      if (!event.target.closest(".theme")) return;
      syncPaletteChecks(list);
      hideMenu(list);
      toggle.setAttribute("aria-expanded", "false");
      toggle.focus();
    });
    if (!nativeMenu) {
      toggle.addEventListener("click", function () {
        pinMenuUnderButton(toggle, list);
      });
      return;
    }
    // mdBook's own theme module owns this same button and writes `display` plus
    // `aria-expanded` imperatively. The popover is the single source of truth, so the
    // click is stopped before it reaches that listener; the native activation still runs,
    // because stopping propagation does not cancel a default action. The theme module's
    // outside-click guard is the only page-level click listener in the book, so stopping
    // the event here costs nothing else.
    document.addEventListener(
      "click",
      function (event) {
        var target = event.target;
        if (!target || !target.closest) return;
        if (target.closest("#mdbook-theme-toggle")) event.stopPropagation();
      },
      true
    );
  }

  // Reading typeface: same pattern as the palette picker, one slot per family declared in
  // theme/book.css (`--font-read`). The choice persists in localStorage and lands on the
  // <html> element, so every component that reads --font-read follows it.
  var TYPEFACE_KEY = "stoch-typeface";
  var TYPEFACES = [
    { id: "sans", label: "Sans" },
    { id: "serif", label: "Serif" },
    { id: "mono", label: "Mono" }
  ];

  function readTypeface() {
    try {
      var saved = localStorage.getItem(TYPEFACE_KEY);
      for (var i = 0; i < TYPEFACES.length; i++) {
        if (TYPEFACES[i].id === saved) return saved;
      }
    } catch (e) {
      /* localStorage may be unavailable; fall through to the default. */
    }
    return TYPEFACES[0].id;
  }

  function applyTypeface(id) {
    if (!document.documentElement) return;
    document.documentElement.setAttribute("data-stoch-font", id);
  }

  // mdBook anchors its own menus at `left: 10px` of the page, far from the buttons they
  // belong to. Where CSS anchor positioning is unavailable this re-pins a menu under its
  // own toggle while it is open; browsers with anchor positioning never call it.
  function pinMenuUnderButton(button, menu) {
    if (!button || !menu || !isMenuOpen(menu)) return;
    var rect = button.getBoundingClientRect();
    // The menu bar carries a backdrop-filter, so it — not the viewport — is the
    // containing block of the fixed menu. Measure that origin instead of assuming it.
    menu.style.left = "0px";
    var origin = menu.getBoundingClientRect().left;
    var width = menu.offsetWidth || 150;
    // Align the menu's inline-end with the toggle's inline-end, the same edge the anchor
    // positioning block uses, so the fallback lands in the same place as the native path.
    var left = Math.min(rect.right - origin - width, window.innerWidth - origin - width - 8);
    menu.style.left = Math.max(0, left) + "px";
    menu.style.right = "auto";
  }

  function injectTypefacePicker() {
    if (document.getElementById("stoch-font-toggle")) return;
    var searchToggle = document.getElementById("mdbook-search-toggle");
    var host = searchToggle ? searchToggle.parentElement : null;
    if (!host || !host.querySelector("#mdbook-theme-toggle")) return;

    // Rendered as a menu-bar plate, and as a list: the control does not navigate anywhere, so
    // an anchor would put a `#` in the address bar every time a reader changed the typeface,
    // and would offer a middle-click "open in new tab" for something that has no destination.
    // The list is what the control is called for — one label now, a menu of faces behind it.
    // `role="button"` keeps the menu-button semantics; the popover is driven through
    // showPopover()/hidePopover() because `popovertarget` only exists on buttons and inputs.
    var toggle = document.createElement("ul");
    toggle.setAttribute("role", "button");
    toggle.tabIndex = 0;
    toggle.id = "stoch-font-toggle";
    toggle.className = "icon-button stoch-font-toggle";
    toggle.title = "Change typeface";
    toggle.setAttribute("aria-label", "Change typeface");
    toggle.setAttribute("aria-haspopup", "true");
    toggle.setAttribute("aria-expanded", "false");
    toggle.setAttribute("aria-controls", "stoch-font-list");
    var toggleLabel = document.createElement("li");
    toggleLabel.textContent = "Aa";
    toggle.appendChild(toggleLabel);

    var list = document.createElement("ul");
    list.id = "stoch-font-list";
    list.className = "theme-popup stoch-font-popup";
    list.setAttribute("aria-label", "Typeface");
    list.setAttribute("role", "menu");
    for (var i = 0; i < TYPEFACES.length; i++) {
      var item = document.createElement("li");
      item.setAttribute("role", "none");
      var option = document.createElement("button");
      option.type = "button";
      option.className = "theme stoch-font-option";
      option.setAttribute("role", "menuitemradio");
      option.setAttribute("aria-checked", "false");
      option.setAttribute("data-font", TYPEFACES[i].id);
      option.textContent = TYPEFACES[i].label;
      item.appendChild(option);
      list.appendChild(item);
    }

    host.insertBefore(toggle, searchToggle);
    host.insertBefore(list, searchToggle);

    function markSelected() {
      var active = document.documentElement.getAttribute("data-stoch-font") || readTypeface();
      var options = list.querySelectorAll(".stoch-font-option");
      for (var o = 0; o < options.length; o++) {
        var isActive = options[o].getAttribute("data-font") === active;
        options[o].classList.toggle("theme-selected", isActive);
        options[o].setAttribute("aria-checked", isActive ? "true" : "false");
      }
    }

    function choose(option) {
      var id = option.getAttribute("data-font");
      applyTypeface(id);
      try {
        localStorage.setItem(TYPEFACE_KEY, id);
      } catch (e) {
        /* non-fatal: the choice simply does not persist. */
      }
      markSelected();
    }

    markSelected();

    // A `role="button"` list has no native activation: Enter and Space must be wired here,
    // or the control is reachable by Tab and dead to the keyboard.
    toggle.addEventListener("keydown", function (event) {
      if (event.key !== "Enter" && event.key !== " ") return;
      event.preventDefault();
      toggle.click();
    });

    if (setupNativeMenu(toggle, list, function () {
      markSelected();
      var selected = list.querySelector(".theme-selected");
      if (selected) selected.focus();
    })) {
      list.addEventListener("click", function (event) {
        var option = event.target.closest(".stoch-font-option");
        if (!option) return;
        choose(option);
        hideMenu(list);
        toggle.focus();
      });
      return;
    }

    // Fallback (no popover API): `display` toggling, a click-outside listener, Escape and
    // the resize re-pin, exactly as before the native path existed.
    list.style.display = "none";

    function closeMenu() {
      hideMenu(list);
      toggle.setAttribute("aria-expanded", "false");
    }

    function openMenu() {
      markSelected();
      showMenu(list);
      pinMenuUnderButton(toggle, list);
      toggle.setAttribute("aria-expanded", "true");
      var selected = list.querySelector(".theme-selected");
      if (selected) selected.focus();
    }

    function toggleMenu() {
      if (isMenuOpen(list)) {
        closeMenu();
        toggle.focus();
      } else {
        openMenu();
      }
    }

    toggle.addEventListener("click", function (event) {
      event.preventDefault();
      toggleMenu();
    });

    toggle.addEventListener("keydown", function (event) {
      if (event.key !== " " && event.key !== "Spacebar") return;
      event.preventDefault();
      toggleMenu();
    });

    list.addEventListener("click", function (event) {
      var option = event.target.closest(".stoch-font-option");
      if (!option) return;
      choose(option);
      closeMenu();
      toggle.focus();
    });

    document.addEventListener("click", function (event) {
      if (!isMenuOpen(list)) return;
      if (toggle.contains(event.target) || list.contains(event.target)) return;
      closeMenu();
    });

    document.addEventListener("keydown", function (event) {
      if (event.key !== "Escape" || !isMenuOpen(list)) return;
      closeMenu();
      toggle.focus();
    });

    window.addEventListener("resize", function () {
      pinMenuUnderButton(toggle, list);
      var themeToggle = document.getElementById("mdbook-theme-toggle");
      var themeList = document.getElementById("mdbook-theme-list");
      if (themeToggle && themeList) pinMenuUnderButton(themeToggle, themeList);
    });
  }

  // Exercise self-checks (B1 grammar, B2 enhancer). mdBook renders a ```check fence as
  // <pre><code class="language-check id=43.3 kind=value accept=0.95|0.20 tol=0.005">, so the
  // whole attribute grammar arrives as space-separated class tokens. This turns the block
  // into a native <details>/<summary> whose reveal is the authored body, and keeps the
  // reader's state in localStorage under one key per exercise. Without JavaScript the
  // authored <pre> stays exactly as authored: nothing is locked behind script.
  // Styling is not done here: the widget ships unstyled and theme/book.css owns every
  // colour, spacing and typography decision through the classes published below.
  var CHECK_KEY_PREFIX = "stoch-check:v1:";
  var CHECK_EMPTY_VERDICT = "—";
  var CHECK_DERIVE_VERDICT = "sem gabarito: a resposta confere com a sua própria leitura";
  var checkWidgets = [];
  var checkResetRegistered = false;

  function checkStore() {
    try {
      return window.localStorage || null;
    } catch (e) {
      return null;
    }
  }

  function readCheckState(id) {
    var store = checkStore();
    if (!store) return null;
    try {
      var raw = store.getItem(CHECK_KEY_PREFIX + id);
      if (!raw) return null;
      var parsed = JSON.parse(raw);
      if (!parsed || typeof parsed !== "object" || parsed.v !== 1) return null;
      var criteria = [];
      if (Object.prototype.toString.call(parsed.criteria) === "[object Array]") {
        for (var i = 0; i < parsed.criteria.length; i++) criteria.push(parsed.criteria[i] === true);
      }
      return {
        v: 1,
        attempt: typeof parsed.attempt === "string" ? parsed.attempt : "",
        revealed: parsed.revealed === true,
        criteria: criteria,
        verdict: typeof parsed.verdict === "string" ? parsed.verdict : "",
        ts: typeof parsed.ts === "number" ? parsed.ts : 0
      };
    } catch (e) {
      return null;
    }
  }

  function writeCheckState(id, state) {
    var store = checkStore();
    if (!store) return;
    try {
      store.setItem(
        CHECK_KEY_PREFIX + id,
        JSON.stringify({
          v: 1,
          attempt: state.attempt,
          revealed: state.revealed,
          criteria: state.criteria,
          verdict: state.verdict,
          ts: Date.now()
        })
      );
    } catch (e) {
      /* private mode or a full quota: the check degrades to session-only. */
    }
  }

  function deleteCheckState(id) {
    var store = checkStore();
    if (!store) return;
    try {
      store.removeItem(CHECK_KEY_PREFIX + id);
    } catch (e) {
      /* non-fatal: the entry simply survives. */
    }
  }

  function clearAllCheckState() {
    var store = checkStore();
    if (!store) return;
    try {
      var doomed = [];
      for (var i = 0; i < store.length; i++) {
        var key = store.key(i);
        if (key && key.indexOf(CHECK_KEY_PREFIX) === 0) doomed.push(key);
      }
      for (var d = 0; d < doomed.length; d++) store.removeItem(doomed[d]);
    } catch (e) {
      /* non-fatal: a partial clear is still a clear. */
    }
  }

  // class="language-check id=43.3 kind=value accept=0.95|0.20 tol=0.005"
  function parseCheckBlock(codeEl) {
    var tokens = [];
    for (var c = 0; c < codeEl.classList.length; c++) {
      var token = codeEl.classList[c];
      if (token !== "language-check") tokens.push(token);
    }
    var attrs = {};
    for (var t = 0; t < tokens.length; t++) {
      var eq = tokens[t].indexOf("=");
      if (eq <= 0) continue;
      var key = tokens[t].slice(0, eq);
      if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(key)) continue;
      attrs[key] = tokens[t].slice(eq + 1);
    }
    var id = attrs.id || "";
    var kind = attrs.kind || "";
    if (!/^\d+\.\d+$/.test(id)) return null;
    if (kind !== "value" && kind !== "derive" && kind !== "rubric") return null;
    var accept = attrs.accept ? attrs.accept.split("|") : [];
    var values = [];
    for (var a = 0; a < accept.length; a++) {
      if (accept[a]) values.push(accept[a]);
    }
    if (kind === "value" && !values.length) return null;
    var tol = parseFloat(attrs.tol);
    if (!isFinite(tol) || tol < 0) tol = 0;
    return { id: id, chapter: id.split(".")[0], ordinal: parseInt(id.split(".")[1], 10), kind: kind, accept: values, tol: tol };
  }

  // Body protocol: `* ` lines are rubric criteria, blank lines separate paragraphs and every
  // other line is reveal text that wraps, so a run of non-blank lines becomes one <p>.
  function splitCheckBody(text) {
    var criteria = [];
    var paragraphs = [];
    var buffer = [];
    var lines = String(text || "").replace(/\r\n?/g, "\n").split("\n");
    var flush = function () {
      if (!buffer.length) return;
      var joined = buffer.join(" ").replace(/\s+/g, " ").trim();
      if (joined) paragraphs.push(joined);
      buffer = [];
    };
    for (var i = 0; i < lines.length; i++) {
      var line = lines[i].replace(/\s+$/, "");
      if (/^[ \t]*\* /.test(line)) {
        flush();
        var criterion = line.replace(/^[ \t]*\* /, "").trim();
        if (criterion) criteria.push(criterion);
        continue;
      }
      if (!line.trim()) {
        flush();
        continue;
      }
      buffer.push(line.trim());
    }
    flush();
    return { criteria: criteria, paragraphs: paragraphs };
  }

  // Normalisation order from the B1 contract: trim, unwrap $…$, drop $, drop thousands
  // separators, drop a trailing %, collapse whitespace, then split on , ; and whitespace.
  function normalizeCheckToken(text) {
    var value = String(text == null ? "" : text).trim();
    var wrapped = value.match(/^\$([\s\S]*)\$$/);
    if (wrapped) value = wrapped[1];
    value = value.replace(/\$/g, "");
    value = value.replace(/(\d),(?=\d{3}(?!\d))/g, "$1");
    value = value.replace(/\s*%\s*$/, "");
    return value.replace(/\s+/g, " ").trim();
  }

  function splitCheckTokens(text) {
    var normalized = normalizeCheckToken(text);
    if (!normalized) return [];
    var parts = normalized.split(/[,;\s]+/);
    var out = [];
    for (var i = 0; i < parts.length; i++) {
      if (parts[i]) out.push(parts[i]);
    }
    return out;
  }

  function checkTokenMatches(attempt, expected, tol) {
    var left = parseFloat(attempt);
    var right = parseFloat(expected);
    if (isFinite(left) && isFinite(right)) return Math.abs(left - right) <= tol + 1e-12;
    return attempt.toLowerCase() === expected.toLowerCase();
  }

  function valueVerdict(attemptText, spec) {
    var tokens = splitCheckTokens(attemptText);
    if (!tokens.length) return CHECK_EMPTY_VERDICT;
    var unmatched = 0;
    for (var i = 0; i < tokens.length; i++) {
      var hit = false;
      for (var j = 0; j < spec.accept.length; j++) {
        if (checkTokenMatches(tokens[i], spec.accept[j], spec.tol)) {
          hit = true;
          break;
        }
      }
      if (!hit) unmatched += 1;
    }
    if (unmatched > 0) return "não fecha";
    var covered = 0;
    for (var k = 0; k < spec.accept.length; k++) {
      for (var m = 0; m < tokens.length; m++) {
        if (checkTokenMatches(spec.accept[k], tokens[m], spec.tol)) {
          covered += 1;
          break;
        }
      }
    }
    return covered === spec.accept.length ? "confere" : "faltam valores";
  }

  function checkVerdictTone(verdict) {
    if (verdict === "confere" || verdict === "completo" || verdict === "marcado: consegui") return "ok";
    if (verdict === "faltam valores" || verdict === "marcado: rever") return "warn";
    if (verdict === "não fecha") return "bad";
    return "idle";
  }

  function nodeFollows(earlier, later) {
    if (!earlier || !later) return false;
    if (earlier === later) return false;
    return (earlier.compareDocumentPosition(later) & 4) === 4;
  }

  function exercisesHeadingBefore(root, node) {
    var headings = root.querySelectorAll("h2");
    var found = null;
    for (var i = 0; i < headings.length; i++) {
      if (!nodeFollows(headings[i], node)) continue;
      found = headings[i];
    }
    if (!found) return null;
    return /^exercises$/i.test((found.textContent || "").trim()) ? found : null;
  }

  function precedingListItem(pre) {
    var node = pre.previousElementSibling;
    while (node) {
      if (node.tagName === "OL" || node.tagName === "UL") {
        var items = node.querySelectorAll("li");
        return items.length ? items[items.length - 1] : null;
      }
      node = node.previousElementSibling;
    }
    return null;
  }

  function listItemOrdinal(item) {
    var list = item.parentElement;
    if (!list) return 0;
    var index = 1;
    for (var i = 0; i < list.children.length; i++) {
      if (list.children[i] === item) {
        index = i + 1;
        break;
      }
    }
    var start = parseInt(list.getAttribute("start"), 10);
    if (!isFinite(start)) start = 1;
    return start + index - 1;
  }

  function itemLeadText(item) {
    if (!item) return "";
    var strong = item.querySelector("strong");
    var text = strong ? strong.textContent : item.textContent;
    return String(text || "").replace(/\s+/g, " ").replace(/:\s*$/, "").trim();
  }

  function createCheckButton(label, className) {
    var button = document.createElement("button");
    button.type = "button";
    button.className = "stoch-check-btn" + (className ? " " + className : "");
    button.textContent = label;
    return button;
  }

  // The reset control sits in its own row right after the Exercises heading, never inside
  // it: the heading's text is the section name, and mdBook's TOC, search snippets and
  // every "is this the Exercises section" test read it.
  function registerCheckReset(heading) {
    if (!heading || checkResetRegistered) return;
    checkResetRegistered = true;
    var row = document.createElement("div");
    row.className = "stoch-check-reset-row";
    row.setAttribute("data-stoch-check-reset-row", "true");
    var button = createCheckButton("Limpar respostas", "stoch-check-reset");
    button.setAttribute("data-stoch-check-reset", "true");
    button.addEventListener("click", function () {
      clearAllCheckState();
      for (var i = 0; i < checkWidgets.length; i++) checkWidgets[i]();
    });
    row.appendChild(button);
    if (heading.parentNode) heading.parentNode.insertBefore(row, heading.nextSibling);
  }

  function hydrateExerciseCheck(root, codeEl) {
    var pre = codeEl.parentElement;
    if (!pre || pre.tagName !== "PRE") return;
    if (codeEl.getAttribute("data-stoch-check") === "hydrated") return;
    var spec = parseCheckBlock(codeEl);
    if (!spec) return;
    var body = splitCheckBody(codeEl.textContent);
    if (!body.paragraphs.length && !body.criteria.length) return;
    if (spec.kind === "rubric" && !body.criteria.length) return;

    var heading = exercisesHeadingBefore(root, pre);
    var item = null;
    if (heading) {
      var candidate = precedingListItem(pre);
      if (candidate && nodeFollows(heading, candidate) && listItemOrdinal(candidate) === spec.ordinal) item = candidate;
    }

    var state = readCheckState(spec.id) || {
      v: 1,
      attempt: "",
      revealed: false,
      criteria: [],
      verdict: "",
      ts: 0
    };
    var hydrating = true;

    // The widget is a div around the disclosure, not the <details> itself: the attempt
    // field, Conferir and the verdict have to stay reachable while the answer is still
    // collapsed, otherwise the only route to the check would be to reveal the answer
    // first. The disclosure keeps the native <details>/<summary> pair inside.
    var widget = document.createElement("div");
    widget.className = "stoch-check";
    widget.setAttribute("data-check-id", spec.id);
    widget.setAttribute("data-kind", spec.kind);
    widget.setAttribute("data-revealed", "false");

    var details = document.createElement("details");
    details.className = "stoch-check-disclosure";
    widget.appendChild(details);

    var summary = document.createElement("summary");
    summary.className = "stoch-check-summary";
    summary.id = "stoch-check-" + spec.id + "-summary";
    var lead = itemLeadText(item);
    summary.textContent = "Exercício " + spec.ordinal + (lead ? " — " + lead : "");
    details.appendChild(summary);

    var bodyEl = document.createElement("div");
    bodyEl.className = "stoch-check-body";
    bodyEl.id = "stoch-check-" + spec.id + "-body";
    for (var p = 0; p < body.paragraphs.length; p++) {
      var para = document.createElement("p");
      para.textContent = normalizeMathText(body.paragraphs[p]);
      bodyEl.appendChild(para);
    }
    details.appendChild(bodyEl);

    var controls = document.createElement("form");
    controls.className = "stoch-check-controls";
    controls.setAttribute("novalidate", "");
    controls.addEventListener("submit", function (event) {
      event.preventDefault();
      if (verify) verify();
    });
    widget.appendChild(controls);

    var verdictEl = document.createElement("p");
    verdictEl.className = "stoch-check-verdict";
    verdictEl.id = "stoch-check-" + spec.id + "-verdict";
    verdictEl.setAttribute("role", "status");
    verdictEl.setAttribute("aria-live", "polite");
    verdictEl.setAttribute("aria-atomic", "true");
    widget.appendChild(verdictEl);

    var input = null;
    var rubricInputs = [];
    var verify = null;
    var saveTimer = null;
    var suppressToggleSave = false;

    function save(immediate) {
      if (saveTimer) {
        clearTimeout(saveTimer);
        saveTimer = null;
      }
      if (immediate) {
        writeCheckState(spec.id, state);
        return;
      }
      saveTimer = setTimeout(function () {
        saveTimer = null;
        writeCheckState(spec.id, state);
      }, 200);
    }

    function criteriaChecked() {
      var count = 0;
      for (var i = 0; i < rubricInputs.length; i++) {
        if (rubricInputs[i].checked) count += 1;
      }
      return count;
    }

    function renderVerdict() {
      var text = state.verdict;
      if (!text) {
        if (spec.kind === "derive") text = CHECK_DERIVE_VERDICT;
        else if (spec.kind === "rubric") text = criteriaChecked() + " de " + rubricInputs.length + " critérios";
        else text = CHECK_EMPTY_VERDICT;
      }
      verdictEl.textContent = text;
      verdictEl.setAttribute("data-tone", checkVerdictTone(state.verdict));
    }

    function setVerdict(text, immediate) {
      state.verdict = text || "";
      renderVerdict();
      save(immediate !== false);
    }

    function reveal() {
      if (details.open) return;
      details.open = true;
    }

    var showBtn = createCheckButton("Mostrar resposta", "stoch-check-reveal");
    controls.appendChild(showBtn);
    showBtn.addEventListener("click", reveal);

    var clearBtn = createCheckButton("Limpar", "stoch-check-clear");
    controls.appendChild(clearBtn);

    if (spec.kind === "value") {
      var field = document.createElement("span");
      field.className = "stoch-check-field";
      var label = document.createElement("label");
      label.className = "stoch-check-label";
      label.setAttribute("for", "stoch-check-" + spec.id + "-attempt");
      label.textContent = "Sua resposta";
      input = document.createElement("input");
      input.type = "text";
      input.id = "stoch-check-" + spec.id + "-attempt";
      input.className = "stoch-check-input";
      input.setAttribute("inputmode", "decimal");
      input.setAttribute("autocomplete", "off");
      input.setAttribute("aria-describedby", verdictEl.id);
      field.appendChild(label);
      field.appendChild(input);
      controls.insertBefore(field, showBtn);
      input.addEventListener("input", function () {
        state.attempt = input.value;
        if (state.verdict && state.verdict !== CHECK_EMPTY_VERDICT) {
          state.verdict = "";
          renderVerdict();
        }
        save(false);
      });
      var checkBtn = createCheckButton("Conferir", "stoch-check-verify");
      controls.insertBefore(checkBtn, showBtn);
      verify = function () {
        state.attempt = input.value;
        setVerdict(valueVerdict(state.attempt, spec), true);
      };
      checkBtn.addEventListener("click", verify);
    }

    if (spec.kind === "derive") {
      var doneBtn = createCheckButton("Consegui", "stoch-check-done");
      var reviewBtn = createCheckButton("Rever", "stoch-check-review");
      controls.insertBefore(doneBtn, showBtn);
      controls.insertBefore(reviewBtn, clearBtn);
      doneBtn.addEventListener("click", function () {
        setVerdict("marcado: consegui", true);
      });
      reviewBtn.addEventListener("click", function () {
        setVerdict("marcado: rever", true);
      });
    }

    if (spec.kind === "rubric") {
      var fieldset = document.createElement("fieldset");
      fieldset.className = "stoch-check-criteria";
      var legend = document.createElement("legend");
      legend.className = "stoch-check-legend";
      legend.textContent = "Critérios (" + body.criteria.length + ")";
      fieldset.appendChild(legend);
      for (var c = 0; c < body.criteria.length; c++) {
        var criterionId = "stoch-check-" + spec.id + "-criterion-" + (c + 1);
        var row = document.createElement("div");
        row.className = "stoch-check-criterion";
        var box = document.createElement("input");
        box.type = "checkbox";
        box.id = criterionId;
        box.className = "stoch-check-box";
        box.value = String(c + 1);
        var boxLabel = document.createElement("label");
        boxLabel.setAttribute("for", criterionId);
        boxLabel.className = "stoch-check-criterion-label";
        boxLabel.textContent = body.criteria[c];
        row.appendChild(box);
        row.appendChild(boxLabel);
        fieldset.appendChild(row);
        rubricInputs.push(box);
      }
      widget.insertBefore(fieldset, controls);
      rubricInputs.forEach(function (box, index) {
        box.addEventListener("change", function () {
          state.criteria = [];
          for (var i = 0; i < rubricInputs.length; i++) state.criteria.push(rubricInputs[i].checked);
          var count = criteriaChecked();
          state.verdict = count === rubricInputs.length ? "completo" : count + " de " + rubricInputs.length + " critérios";
          renderVerdict();
          save(true);
        });
        if (state.criteria[index] === true) box.checked = true;
      });
    }

    // `open` lives on the disclosure; `data-revealed` mirrors it on the widget so a
    // stylesheet never has to reach across the wrapper to know the state.
    details.addEventListener("toggle", function () {
      state.revealed = details.open === true;
      widget.setAttribute("data-revealed", state.revealed ? "true" : "false");
      if (hydrating) return;
      if (suppressToggleSave) {
        suppressToggleSave = false;
        return;
      }
      save(true);
    });

    // Space toggles a <summary> natively; Enter does not in current Chrome, and the
    // accessibility contract asks for both, so Enter is wired here and nothing else is.
    summary.addEventListener("keydown", function (event) {
      if (event.key !== "Enter" && event.key !== "Spacebar") return;
      if (event.key === "Spacebar") return;
      event.preventDefault();
      details.open = !details.open;
    });

    widget.addEventListener("keydown", function (event) {
      if (event.key !== "Escape" || !details.open) return;
      event.preventDefault();
      details.open = false;
      summary.focus();
    });

    // `persist` is false only for the page-wide reset, which already wiped every key.
    function resetWidget(persist) {
      if (saveTimer) {
        clearTimeout(saveTimer);
        saveTimer = null;
      }
      if (persist) deleteCheckState(spec.id);
      state = { v: 1, attempt: "", revealed: false, criteria: [], verdict: "", ts: 0 };
      if (details.open) {
        suppressToggleSave = true;
        details.open = false;
      }
      widget.setAttribute("data-revealed", "false");
      if (input) input.value = "";
      for (var i = 0; i < rubricInputs.length; i++) rubricInputs[i].checked = false;
      renderVerdict();
    }

    clearBtn.addEventListener("click", function () {
      resetWidget(true);
      summary.focus();
    });

    if (input) input.value = state.attempt || "";
    if (state.revealed) details.open = true;
    widget.setAttribute("data-revealed", details.open ? "true" : "false");
    renderVerdict();
    hydrating = false;

    // A printed chapter carries its solutions: the answer opens for the print job and the
    // reader's own state is neither written nor changed.
    var printOpened = false;
    window.addEventListener("beforeprint", function () {
      if (details.open) return;
      printOpened = true;
      suppressToggleSave = true;
      details.open = true;
    });
    window.addEventListener("afterprint", function () {
      if (!printOpened) return;
      printOpened = false;
      suppressToggleSave = true;
      details.open = false;
    });

    codeEl.setAttribute("data-stoch-check", "hydrated");
    if (item) {
      item.appendChild(widget);
      if (pre.parentNode) pre.parentNode.removeChild(pre);
    } else {
      pre.parentNode.replaceChild(widget, pre);
    }
    if (heading) registerCheckReset(heading);
    checkWidgets.push(function () {
      resetWidget(false);
    });
  }

  function setupExerciseChecks(root) {
    var blocks = root.querySelectorAll("pre > code.language-check");
    if (!blocks.length) return;
    checkWidgets = [];
    checkResetRegistered = false;
    for (var i = 0; i < blocks.length; i++) {
      try {
        hydrateExerciseCheck(root, blocks[i]);
      } catch (e) {
        /* a block that cannot be parsed stays an authored <pre>; never throw out of the loop. */
      }
    }
  }

  function prepareDocumentForMathAndTables() {
    injectSkipLink();
    injectTopMenuLogo();
    initThemeLabels();
    injectTypefacePicker();

    var root = document.querySelector("main") || document.body;
    if (!root) return;

    rewriteRepositoryLinks(root);

    var mathContainers = root.querySelectorAll("p, li, td, th, h1, h2, h3, h4, h5, h6, figcaption");
    for (var mc = 0; mc < mathContainers.length; mc++) {
      var el = mathContainers[mc];
      var html = el.innerHTML;
      if (html.indexOf("$") === -1 || html.indexOf("<em>") === -1) continue;
      var codeSlots = [];
      var maskedHtml = html.replace(/<(code|pre)\b[^>]*>[\s\S]*?<\/\1>/gi, function (m) {
        codeSlots.push(m);
        return "\x00CODE_SLOT_" + (codeSlots.length - 1) + "\x00";
      });
      var fixedHtml = maskedHtml.replace(/(\$\$[\s\S]*?\$\$|\$[^$\n]+\$)/g, function (seg) {
        return seg.replace(/<\/?em>/gi, "_");
      });
      if (fixedHtml !== maskedHtml) {
        for (var cs = 0; cs < codeSlots.length; cs++) {
          fixedHtml = fixedHtml.replace("\x00CODE_SLOT_" + cs + "\x00", codeSlots[cs]);
        }
        el.innerHTML = fixedHtml;
      }
    }

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

    // The self-checks are hydrated after the two passes above: the math repair rewrites the
    // innerHTML of an exercise item that mixes `$…$` with <em>, which would rebuild the
    // widget and drop every listener. Hydrating last keeps the nodes we wire up.
    setupExerciseChecks(root);

    enhanceResponsiveTables(root);
    enhanceScrollablePreBlocks(root);

    var svgImgs = root.querySelectorAll('p > img[src*="figures/"][src$=".svg"]');
    for (var k = 0; k < svgImgs.length; k++) {
      (function (img) {
        var parentP = img.parentElement;
        if (!parentP || parentP.children.length !== 1) return;
        var altText = (img.getAttribute("alt") || "").trim();
        var srcAttr = img.getAttribute("src") || "";
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
        if (typeof window.fetch === "function" && typeof window.DOMParser === "function" && srcAttr) {
          window
            .fetch(srcAttr)
            .then(function (resp) {
              return resp.ok ? resp.text() : Promise.reject();
            })
            .then(function (svgMarkup) {
              var doc = new window.DOMParser().parseFromString(svgMarkup, "image/svg+xml");
              var svgEl = doc && doc.documentElement;
              if (svgEl && svgEl.tagName && svgEl.tagName.toLowerCase() === "svg" && img.parentNode === fig) {
                var imported = document.importNode(svgEl, true);
                if (altText && !imported.getAttribute("aria-label")) {
                  imported.setAttribute("aria-label", altText);
                }
                fig.replaceChild(imported, img);
              }
            })
            .catch(function () {});
        }
      })(svgImgs[k]);
    }

  /* Chapter emblem motion. Only the seal moves. Every chapter carries its mark as inline SVG,
     so the page can number its parts directly: the emblem's groups are walked in document
     order and each child gets an index, which the stylesheet turns into a stagger. The mark
     settles once when it enters the viewport and stops there — motion here is a calibration
     pass, not a loop. Data figures are left alone: an instrument's plate reports a
     measurement, and a report does not perform.

     `js-plates` is added only after the indices are written, before anything is hidden, so a
     script that dies earlier leaves the emblem fully drawn instead of blank. */
  function setupEmblemSettle(root) {
    var figures = root.querySelectorAll(".stoch-logo-figure");
    if (!figures.length) {
      return;
    }

    var emblems = [];
    for (var f = 0; f < figures.length; f++) {
      var svg = figures[f].querySelector("svg");
      if (!svg) continue;
      var n = 0;
      for (var g = 0; g < svg.children.length; g++) {
        var group = svg.children[g];
        if (group.tagName.toLowerCase() !== "g") continue;
        for (var k = 0; k < group.children.length; k++) {
          /* Capped so a dense emblem does not spend a second arriving; the cap is also what
             keeps the last label from landing well after the reader has started reading. */
          group.children[k].style.setProperty("--i", String(Math.min(n, 24)));
          n += 1;
        }
      }
      emblems.push(figures[f]);
    }

    if (!emblems.length) {
      return;
    }

    var settle = function (figure) {
      figure.classList.add("is-settled");
    };

    document.documentElement.classList.add("js-plates");

    if (typeof window.IntersectionObserver !== "function") {
      for (var r = 0; r < emblems.length; r++) settle(emblems[r]);
      return;
    }

    var observer = new window.IntersectionObserver(
      function (entries) {
        for (var e = 0; e < entries.length; e++) {
          if (!entries[e].isIntersecting) continue;
          settle(entries[e].target);
          observer.unobserve(entries[e].target);
        }
      },
      { rootMargin: "0px 0px -6% 0px", threshold: 0.05 }
    );

    for (var o = 0; o < emblems.length; o++) {
      observer.observe(emblems[o]);
    }

    /* An emblem that never intersects — a page opened at an anchor below it, a viewport the
       observer never reports, a tab kept hidden — must not stay invisible. */
    window.setTimeout(function () {
      for (var t = 0; t < emblems.length; t++) {
        settle(emblems[t]);
      }
    }, 1400);
  }

    setupMermaidDiagrams(root);
    setupSvgZoomModals(root);
    setupEmblemSettle(root);
    dismissPageLoader();
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
        prev ? prev.textContent.trim() + " — Diagram" : "Stochology Diagram"
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

  // Paint with the saved typeface before the picker is wired up.
  applyTypeface(readTypeface());
  injectTopMenuLogo();

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", prepareDocumentForMathAndTables);
  } else {
    prepareDocumentForMathAndTables();
  }

  // Safety fallback: dismiss loader under any condition within 1.2s so content is never blocked
  setTimeout(dismissPageLoader, 1200);
  if (typeof window !== "undefined") {
    window.addEventListener("load", dismissPageLoader);
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

  // MathJax is vendored beside the theme, fonts included, so the algebra on the page does not
  // depend on a third party answering at read time. The CDN stays only as a fallback for a
  // partial deploy, where the theme directory was published without its payload.
  var s = document.createElement("script");
  s.src = "mathjax/tex-chtml.js";
  s.async = true;
  s.onerror = function () {
    var fallback = document.createElement("script");
    fallback.src = "https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-chtml.js";
    fallback.async = true;
    document.head.appendChild(fallback);
  };
  document.head.appendChild(s);
})();
