# Third-party notices

The repository ships one vendored third-party component. Everything else — the
manuscript, the theme stylesheet, the template overrides, the icons and the
MathJax loader at the repository root — is the work of this project and carries
the licenses stated in [`LICENSE`](LICENSE) (MIT, software and theme assets) and
[`LICENSE-TEXT`](LICENSE-TEXT) (CC BY 4.0, the manuscript text).

## MathJax

- **Component:** MathJax 3.2.x, typesetting payload, vendored as the subset
  `theme/mathjax/tex-chtml.js` plus the CHTML web fonts under
  `theme/mathjax/output/chtml/fonts/`.
- **Upstream:** <https://github.com/mathjax/MathJax>
- **License:** Apache License 2.0. The full text travels with the payload at
  [`theme/mathjax/LICENSE`](theme/mathjax/LICENSE).
- **Why it is vendored:** the rendered pages must typeset their formulas from
  this origin, so a reader opening the book offline, or from a network that
  cannot reach a CDN, still sees the mathematics. The publication workflow copies
  the payload into the rendered tree (`book/mathjax`) for that reason.

No other third-party code, font, image or data file is redistributed here.
