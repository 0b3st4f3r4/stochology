# 22. The Four Canonical Epistemological Cards


## 22.1 Comparative Overview of the Four Canonical Cards

Before reading the full YAML listings below, compare how the four cards span all three ontological layers ($L\_1, L\_2, L\_3$), all four uncertainty regimes, and four real empirical domains:

| Card File | Primary Layer & Uncertainty | Real Dataset (`data/*.csv`) | Cost-Loss Threshold $p^{\ast} = \frac{C}{\eta L}$ | Calibrated Forecast $p$ | Declared Action & Why |
| --- | --- | --- | --- | --- | --- |
| **`card_01_atlantic_storms_evacuation.yaml`** | `L1` / `epistemic_specification` | `data/atlantic_storms.csv` ($n = 20\text{,}778$, $n\_{\mathrm{eff}} = 1\text{,}811.45$) | $\frac{12\text{,}000}{0.80 \times 125\text{,}000} = 0.12000$ | $p = 0.09800$ | **`do_not_protect`** ($p = 0.09800 \le p^{\ast} = 0.12000$): calibrated 85-kt threat is below the evacuation threshold. |
| **`card_02_bank_queue_staffing.yaml`** | `L2` / `aleatory` | `data/bank_calls.csv` ($n = 27\text{,}716$, $n\_{\mathrm{eff}} = 980.70$) | $\frac{1\text{,}600}{0.80 \times 25\text{,}000} = 0.08000$ | $p = 0.09911$ | **`protect`** ($p = 0.09911 > p^{\ast} = 0.08000$): once Jensen's Gap and Moulton-Campbell tail clustering are accounted for, surge risk exceeds $p^{\ast}$. |
| **`card_03_nyc_flights_holding.yaml`** | `L3` / `knightian` | `data/nyc_flights_2013.csv` ($n = 138\text{,}375$, $n\_{\mathrm{eff}} = 1\text{,}364.00$) | $\frac{2\text{,}700}{0.75 \times 60\text{,}000} = 0.06000$ | $p = 0.09800$ | **`protect`** ($p = 0.09800 > p^{\ast} = 0.06000$): under severe cascade loss $L = USD 60\text{,}000$, $p^{\ast} = 6\%$ is crossed and Info-Gap + Wald safeguard holds. |
| **`card_04_dpi_civil_defense_triage.yaml`** | `L1` / `epistemic_parameter` | `data/cps_observational.csv` ($n = 15\text{,}992$, $n\_{\mathrm{eff}} = 1\text{,}583.00$) | $\frac{1\text{,}800}{0.75 \times 20\text{,}000} = 0.12000$ | $p = 0.17059$ | **`protect`** ($p = 0.17059 > p^{\ast} = 0.12000$): after Meng `MNAR` adjustment ($\rho\_{R,Y} = +0.1334$), Lee bounds $[838.7, 2\text{,}007.2]$, and Chouldechova audit, mobile-outreach dispatch is optimal. |

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1140" height="240" viewBox="0 0 1140 240" fill="none" role="img" aria-labelledby="title-fig-13-cards">
  <title id="title-fig-13-cards">Comparative Cost-Loss Geometry (p vs p*) Across the Four Canonical Epistemological Cards</title>
  <defs>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1138" height="238" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>
  <rect x="16" y="22" width="264" height="196" rx="12" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2.2"/>
  <text x="30" y="48" fill="var(--sky)" class="mono" font-size="11" font-weight="800">CARD 01 · STORMS (L₁)</text>
  <text x="30" y="72" fill="var(--ink)" font-size="14" font-weight="800">Coastal Evacuation</text>
  <text x="30" y="100" fill="var(--muted)" class="mono" font-size="11">p = 0.09800 ≤ p* = 0.12000</text>
  <text x="30" y="124" fill="var(--muted)" class="mono" font-size="11">n = 20,778 → n_eff = 1,811</text>
  <text x="30" y="148" fill="var(--muted)" class="mono" font-size="11">EVPI = $257,928 · E₃₀ = 324k</text>
  <text x="30" y="190" fill="var(--sky)" class="mono" font-size="11.5" font-weight="800">Act: do_not_protect (OK)</text>
  <rect x="296" y="22" width="264" height="196" rx="12" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2.2"/>
  <text x="310" y="48" fill="var(--violet)" class="mono" font-size="11" font-weight="800">CARD 02 · BANK QUEUES (L₂)</text>
  <text x="310" y="72" fill="var(--ink)" font-size="14" font-weight="800">Overflow Shift Staffing</text>
  <text x="310" y="100" fill="var(--muted)" class="mono" font-size="11">p = 0.09911 &gt; p* = 0.08000</text>
  <text x="310" y="124" fill="var(--muted)" class="mono" font-size="11">n = 27,716 → n_eff = 980.7</text>
  <text x="310" y="148" fill="var(--muted)" class="mono" font-size="11">EVPI = $216,214 · E₃₀ = 2,786</text>
  <text x="310" y="190" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">Act: protect (Approved)</text>
  <rect x="576" y="22" width="264" height="196" rx="12" fill="var(--elevated)" stroke="var(--emerald)" stroke-width="2.2"/>
  <text x="590" y="48" fill="var(--emerald)" class="mono" font-size="11" font-weight="800">CARD 03 · NYC FLIGHTS (L₃)</text>
  <text x="590" y="72" fill="var(--ink)" font-size="14" font-weight="800">Ground-Holding Buffer</text>
  <text x="590" y="100" fill="var(--muted)" class="mono" font-size="11">p = 0.09800 &gt; p* = 0.06000</text>
  <text x="590" y="124" fill="var(--muted)" class="mono" font-size="11">n = 138,375 → n_eff = 1,364</text>
  <text x="590" y="148" fill="var(--muted)" class="mono" font-size="11">EVPI = $135,285 · Wald OK</text>
  <text x="590" y="190" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">Act: protect (Approved)</text>
  <rect x="856" y="22" width="268" height="196" rx="12" fill="var(--elevated)" stroke="var(--amber)" stroke-width="2.2"/>
  <text x="870" y="48" fill="var(--amber)" class="mono" font-size="11" font-weight="800">CARD 04 · DPI TRIAGE (L₁)</text>
  <text x="870" y="72" fill="var(--ink)" font-size="14" font-weight="800">Civil-Defense Outreach</text>
  <text x="870" y="100" fill="var(--muted)" class="mono" font-size="11">p = 0.17059 &gt; p* = 0.12000</text>
  <text x="870" y="124" fill="var(--muted)" class="mono" font-size="11">n = 15,992 → n_eff = 1,583</text>
  <text x="870" y="148" fill="var(--muted)" class="mono" font-size="11">Meng n_eff = 2,843 · Lee OK</text>
  <text x="870" y="190" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">Act: protect (Approved)</text>
  </svg>
  <figcaption><strong>Figure 22.1 — Comparative Cost-Loss Decision Geometry Across the Four Canonical Epistemological Cards</strong></figcaption>
</figure>

<div class="stoch-lab"
     data-lab="gigerenzer-bayes"
     data-title="Canonical Cards Natural-Frequency &amp; Deciban Triage Explorer"
     data-subtitle="Simulate the Step-8 Gigerenzer integer cohort (N = 10,000) and Turing deciban threshold across Cards 01–04">
</div>

---

## 22.2 `card_01_atlantic_storms_evacuation.yaml` (Atmospheric Physics & Coastal Evacuation)

This card models 6-hour synoptic rapid-intensification advisories across $n = 20\text{,}778$ NOAA Atlantic tropical cyclone track records (`data/atlantic_storms.csv`). The card records the Moulton-Campbell cluster adjustment across multi-point storm tracks ($\tilde{m} = 45.20 \implies n\_{\mathrm{eff}} = 1\text{,}811.45$), the isotonic CORP-PAV decomposition ($\mathrm{BS} = 0.03210 = 0.00578 - 0.06183 + 0.08815$), and the rational decision `do_not_protect` whenever $p = 0.09800 \le p^{\ast} = 0.12000$:

~~~yaml
{{#include ../cards/card_01_atlantic_storms_evacuation.yaml}}
~~~

---

## 22.3 `card_02_bank_queue_staffing.yaml` (Queueing Convexity & Shift Staffing)

This card models 5-minute customer call arrivals across $164$ business days (`data/bank_calls.csv`, $n = 27\text{,}716$). It illustrates **Bridge $L\_2 \to L\_3$** (Prohibition 2, with Jensen's Gap $\Delta\_J = +9\text{,}033\text{,}837.19$, $\approx 0.85\%$ of $f(\mathbb{E}[X])$ under the illustrative cost $f(x) = x^2$) and **Invariant $\Phi\_2$** (where $\tilde{m} = 169.0$ and $\rho\_I(0.90) = 0.1623$ inflate variance by $\mathrm{DEFF} = 28.26\times$, reducing $n = 27\text{,}716$ to $n\_{\mathrm{eff}} = 980.70$):

~~~yaml
{{#include ../cards/card_02_bank_queue_staffing.yaml}}
~~~

---

## 22.4 `card_03_nyc_flights_holding.yaml` (Bitemporal Aviation Governance & Performative Holding)

This card governs ground-holding and gate-buffering decisions across $n = 138\text{,}375$ New York flight departures (`data/nyc_flights_2013.csv`). It operates on **Layer $L\_3$** under **Knightian** uncertainty, auditing performative air-traffic feedback ($r\_{\mathrm{RRM}} = 0.30 < 1$), Info-Gap robustness ($\hat{\alpha} = 2.80$) with a verified Global Wald Maximin floor, and bitemporal/drift exit clauses:

~~~yaml
{{#include ../cards/card_03_nyc_flights_holding.yaml}}
~~~

---

## 22.5 `card_04_dpi_civil_defense_triage.yaml` (Digital Public Infrastructure & Civil-Defense Triage)

This card governs daily mobile-outreach and emergency-relief dispatch across $n = 15\text{,}992$ administrative household records (`data/cps_observational.csv` paired with randomized ground-truth `data/nsw_experimental.csv`). It demonstrates **Meng's Identity** ($\rho\_{R,Y} = +0.1334 \implies n\_{\mathrm{eff,Meng}} = 2\text{,}842.70$), **Manski `MNAR` bounds** ($[5\text{,}463.2,\ 13\text{,}835.3]$) and **Lee's (2009) sharp trimming bounds** ($[838.7,\ 2\text{,}007.2]$), Moulton-Campbell census-tract clustering ($\tilde{m} = 95.65, \mathrm{DEFF} = 10.10 \implies n\_{\mathrm{eff}}(0.90) = 1\text{,}583$), cost-loss boundary $p^{\ast} = \frac{1\text{,}800}{0.75 \times 20\text{,}000} = 0.12000$, and **Chouldechova/KMR** fairness (`who_pays_for_error`, $\Phi\_4$):

~~~yaml
{{#include ../cards/card_04_dpi_civil_defense_triage.yaml}}
~~~
