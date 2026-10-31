# Appendix C — Empirical Datasets and Bibliography

## C.1 Sealed Empirical Datasets (`data/*.csv` & `data/SHA256SUMS`)

Every empirical measurement reported in this book is computed directly from the 14 real-world CSV datasets sealed in [`data/SHA256SUMS`](../data/SHA256SUMS) and documented in [`data/SOURCES.md`](../data/SOURCES.md):

| Dataset File | Rows ($n$) | Provenance & Domain | Chapter Laboratory |
| :--- | :---: | :--- | :---: |
| `data/nsw_experimental.csv` | $445$ | Robert LaLonde (1986) / Dehejia & Wahba (1999) National Supported Work (NSW) randomized job-training trial (`treat`, `re74`, `re75`, `re78`). | Ch. 3 |
| `data/cps_observational.csv` | $15{,}992$ | Current Population Survey (CPS-1) non-experimental comparison cohort paired with NSW (`age`, `educ`, `black`, `hisp`, `married`, `nodegree`, `re74`, `re75`, `re78`). | Ch. 3, Ch. 5 |
| `data/danish_fire.csv` | $2{,}167$ | Copenhagen Reinsurance Danish fire insurance claims (1980–1990) exceeding $1\text{M}$ DKK (`loss_m_dkk`, McNeil 1997). | Ch. 4 |
| `data/nyc_flights_2013.csv` | $5{,}000$ | U.S. Bureau of Transportation Statistics (BTS) NYC flight departure and arrival delays (2013 subsample). | Ch. 4 |
| `data/nyc_flights_2023.csv` | $5{,}000$ | U.S. BTS NYC flight departure and arrival delays (2023 regime comparison). | Ch. 4 |
| `data/nyc_flights_2013_h1.csv` | $161{,}402$ | Full H1 2013 BTS NYC flight operations archive. | Sealed Archive |
| `data/nyc_flights_2023_h1.csv` | $209{,}129$ | Full H1 2023 BTS NYC flight operations archive. | Sealed Archive |
| `data/houston_flights_2011.csv` | $227{,}496$ | U.S. BTS Houston (IAH/HOU) 2011 flight operations archive. | Sealed Archive |
| `data/bank_calls.csv` | $27{,}716$ | Anonymous Bank Call Center timestamped call arrivals and service durations across 12 months (Brown et al., 2005). | Ch. 5 |
| `data/rtdsm_routput_vintages.csv` | $241 \times 241$ | Federal Reserve Bank of Philadelphia Real-Time Data Set for Macroeconomists (`ROUTPUT`, Croushore & Stark, 2001): quarterly real GDP/GNP across 241 historical vintages (`1965:Q4` to `2025:Q4`). | Ch. 5 |
| `data/atlantic_storms.csv` | $20{,}778$ | NOAA NHC HURDAT2 Atlantic tropical cyclone 6-hourly best-track observations (`1975–2021`, `wind`, `pressure`, `lat`, `long`). | Ch. 6 |
| `data/djia_1885_2023.csv` | $37{,}930$ | Dow Jones Industrial Average daily closing levels and log-returns from 1885 to 2023 (Stooq / MeasuringWorth). | Ch. 7 |
| `data/vic_elec.csv` | $52{,}608$ | Australian Energy Market Operator (AEMO) half-hourly electricity demand (`MW`) and temperature in Victoria (2012–2014). | Ch. 7 |
| `data/toronto_arrests.csv` | $5{,}226$ | Toronto Police Service arrest and release outcomes by demographic and neighborhood records (Fox & Weisberg `carData::Arrests`). | Ch. 8 |

---

## C.2 Consolidated Bibliography

- **Adorno, T. W.** (1966). *Negative Dialektik*. Suhrkamp.
- **Aristotle** (*c.* 335 BCE). *Nicomachean Ethics* (Books II & VI).
- **Ashby, W. R.** (1956). *An Introduction to Cybernetics*. Chapman & Hall.
- **Baker, R. D., & McHale, I. G.** (2013). Optimal betting under parameter uncertainty: Improving the Kelly criterion. *Decision Analysis*, 10(3), 189–199.
- **Baron, J., & Hershey, J. C.** (1988). Outcome bias in decision evaluation. *Journal of Personality and Social Psychology*, 54(4), 569–579.
- **Beer, S.** (1972). *Brain of the Firm*. Allen Lane / Penguin.
- **Ben-Haim, Y.** (2006). *Info-Gap Decision Theory: Decisions Under Severe Uncertainty* (2nd ed.). Academic Press.
- **Bennett, A., & Checkel, J. T.** (Eds.). (2015). *Process Tracing: From Metaphor to Analytic Tool*. Cambridge University Press.
- **Bernoulli, J.** (1713). *Ars Conjectandi*. Thurneysen Brothers, Basel.
- **Breiman, L.** (1961). Optimal gambling systems for favorable games. *Proceedings of the Fourth Berkeley Symposium on Mathematical Statistics and Probability*, 1, 65–78.
- **Brown, L., Gans, N., Mandelbaum, A., Sakov, A., Shen, H., Zeltyn, S., & Zhao, L.** (2005). Statistical analysis of a telephone call center: A queueing-science perspective. *Journal of the American Statistical Association*, 100(469), 36–50.
- **Campbell, D. T.** (1979). Assessing the impact of planned social change. *Evaluation and Program Planning*, 2(1), 67–90.
- **Canguilhem, G.** (1943/1966). *Le Normal et le Pathologique*. Presses Universitaires de France.
- **Chao, A.** (1984). Nonparametric estimation of the number of classes in a population. *Scandinavian Journal of Statistics*, 11(4), 265–270.
- **Chouldechova, A.** (2017). Fair prediction with disparate impact: A study of bias in recidivism prediction instruments. *Big Data*, 5(2), 153–163.
- **Cicero, M. T.** (45 BCE). *De Finibus Bonorum et Malorum* (Book III.22, Antipater of Tarsus's archer).
- **Croushore, D., & Stark, T.** (2001). A real-time data set for macroeconomists. *Journal of Econometrics*, 105(1), 111–130.
- **Desrosières, A.** (1993). *La Politique des grands nombres: Histoire de la raison statistique*. La Découverte.
- **Dimitriadis, T., Gneiting, T., & Jordan, A. I.** (2021). Stable reliability diagrams for probabilistic classifiers. *Proceedings of the National Academy of Sciences (PNAS)*, 118(8), e2016191118.
- **Douglas, H.** (2000). Inductive risk and values in science. *Philosophy of Science*, 67(4), 559–579.
- **Duhem, P.** (1906). *La Théorie physique: son objet et sa structure*. Chevalier & Rivière.
- **Fantl, J., & McGrath, M.** (2009). *Knowledge in an Uncertain World*. Oxford University Press.
- **Fricker, M.** (2007). *Epistemic Injustice: Power and the Ethics of Knowing*. Oxford University Press.
- **Georgescu-Roegen, N.** (1971). *The Entropy Law and the Economic Process*. Harvard University Press.
- **Gigerenzer, G., & Hoffrage, U.** (1995). How to improve Bayesian reasoning without instruction: Frequency formats. *Psychological Review*, 102(4), 684–704.
- **Gneiting, T., & Raftery, A. E.** (2007). Strictly proper scoring rules, prediction, and estimation. *Journal of the American Statistical Association*, 102(477), 359–378.
- **Goodhart, C. A. E.** (1975). Problems of monetary management: The U.K. experience. *Papers in Monetary Economics*, Reserve Bank of Australia.
- **Hacking, I.** (2002). *Historical Ontology*. Harvard University Press.
- **Hansson, S. O.** (2013). *The Ethics of Risk: Ethical Analysis in an Uncertain World*. Palgrave Macmillan.
- **Harding, S.** (1991). *Whose Science? Whose Knowledge? Thinking from Women's Lives*. Cornell University Press.
- **Hill, B. M.** (1975). A simple general approach to inference about the tail of a distribution. *The Annals of Statistics*, 3(5), 1163–1174.
- **Hippocrates** (*c.* 400 BCE). *On Ancient Medicine* (*De Vetere Medicina*) & *Prognostic*.
- **Holling, C. S.** (1973). Resilience and stability of ecological systems. *Annual Review of Ecology and Systematics*, 4, 1–23.
- **Holmström, B.** (1979). Moral hazard and observability. *The Bell Journal of Economics*, 10(1), 74–91.
- **Holmström, B., & Milgrom, P.** (1991). Multitask principal–agent analyses: Incentive contracts, asset ownership, and job design. *Journal of Law, Economics, & Organization*, 7, 24–52.
- **Horvitz, D. G., & Thompson, D. J.** (1952). A generalization of sampling without replacement from a finite universe. *Journal of the American Statistical Association*, 47(260), 663–685.
- **Hudgens, M. G., & Halloran, M. E.** (2008). Toward causal inference with interference. *Journal of the American Statistical Association*, 103(482), 832–842.
- **Imbens, G. W., & Manski, C. F.** (2004). Confidence intervals for partially identified parameters. *Econometrica*, 72(6), 1845–1857.
- **Innes, R. D.** (1990). Limited liability and incentive contracting with ex-ante action choices. *Journal of Economic Theory*, 52(1), 45–67.
- **Itô, K.** (1951). On a formula concerning stochastic differentials. *Nagoya Mathematical Journal*, 3, 55–65.
- **Jensen, J. L. W. V.** (1906). Sur les fonctions convexes et les inégalités entre les valeurs moyennes. *Acta Mathematica*, 30(1), 175–193.
- **Kelly, J. L., Jr.** (1956). A new interpretation of information rate. *Bell System Technical Journal*, 35(4), 917–926.
- **Kennedy, M. C., & O'Hagan, A.** (2001). Bayesian calibration of computer models. *Journal of the Royal Statistical Society: Series B*, 63(3), 425–464.
- **Keynes, J. M.** (1921). *A Treatise on Probability*. Macmillan.
- **Kimball, A. W.** (1957). Errors of the third kind in statistical consulting. *Journal of the American Statistical Association*, 52(278), 133–142.
- **Kingman, J. F. C.** (1961). The single server queue in heavy traffic. *Mathematical Proceedings of the Cambridge Philosophical Society*, 57(4), 902–904.
- **Kish, L.** (1965). *Survey Sampling*. John Wiley & Sons.
- **Kleinberg, J., & Raghavan, M.** (2020). How do classifiers induce agents to invest effort strategically? *ACM Transactions on Economics and Computation*, 8(4), 1–23.
- **Lakatos, I.** (1970). Falsification and the methodology of scientific research programmes. In I. Lakatos & A. Musgrave (Eds.), *Criticism and the Growth of Knowledge* (pp. 91–196). Cambridge University Press.
- **LaLonde, R. J.** (1986). Evaluating the econometric evaluations of training programs with experimental data. *The American Economic Review*, 76(4), 604–620.
- **Lee, D. S.** (2009). Training, wages, and sample selection: Estimating sharp bounds on treatment effects. *Review of Economic Studies*, 76(3), 1071–1102.
- **Lindley, D. V.** (1985). *Making Decisions* (2nd ed.). John Wiley & Sons.
- **List, C., & Pettit, P.** (2011). *Group Agency: The Possibility, Design, and Status of Corporate Agents*. Oxford University Press.
- **Lucas, R. E., Jr.** (1976). Econometric policy evaluation: A critique. *Carnegie-Rochester Conference Series on Public Policy*, 1, 19–46.
- **Lum, K., Price, M. E., & Banks, D.** (2013). Applications of multiple systems estimation in human rights research. *The American Statistician*, 67(4), 191–200.
- **MacKenzie, D.** (2006). *An Engine, Not a Camera: How Financial Models Shape Markets*. MIT Press.
- **Manheim, D., & Garrabrant, S.** (2018). Categorizing variants of Goodhart's Law. *arXiv:1803.04585*.
- **Manski, C. F.** (1990). Nonparametric bounds on treatment effects. *The American Economic Review*, 80(2), 319–323.
- **Manski, C. F.** (2003). *Partial Identification of Probability Distributions*. Springer.
- **Mayo, D. G.** (2018). *Statistical Inference as Severe Testing: How to Get Beyond the Statistics Wars*. Cambridge University Press.
- **McEwen, B. S.** (1998). Protective and damaging effects of stress mediators. *New England Journal of Medicine*, 338(3), 171–179.
- **Meng, X.-L.** (2018). Statistical paradoxes in big data: Law of large populations, big data paradox, and the 2016 US presidential election. *The Annals of Applied Statistics*, 12(2), 685–726.
- **Moulton, B. R.** (1986). Random group effects and the precision of regression estimates. *Journal of Econometrics*, 32(3), 385–397.
- **Nagel, T.** (1979). Moral luck. In *Mortal Questions* (pp. 24–38). Cambridge University Press.
- **Peirce, C. S.** (1899/1931). First rule of reason (*Collected Papers*, CP 1.135). Harvard University Press.
- **Perdomo, J., Zrnic, T., Mendler-Dünner, C., & Hardt, M.** (2020). Performative prediction. *Proceedings of the 37th International Conference on Machine Learning (ICML)*, PMLR 119, 7599–7609.
- **Peters, J., Bühlmann, P., & Meinshausen, N.** (2016). Causal inference by using invariant prediction: Identification and confidence intervals. *Journal of the Royal Statistical Society: Series B*, 78(5), 947–1012.
- **Peters, O.** (2019). The ergodicity problem in economics. *Nature Physics*, 15(12), 1216–1221.
- **Peterson, A. V., Jr.** (1976). Bounds for a joint distribution function with fixed sub-distribution functions: Application to competing risks. *Proceedings of the National Academy of Sciences (PNAS)*, 73(1), 11–13.
- **Plato** (*c.* 360 BCE). *Philebus* (55e–56c).
- **Popper, K. R.** (1959). *The Logic of Scientific Discovery*. Hutchinson.
- **Porter, T. M.** (1995). *Trust in Numbers: The Pursuit of Objectivity in Science and Public Life*. Princeton University Press.
- **Proctor, R. N., & Schiebinger, L.** (Eds.). (2008). *Agnotology: The Making and Unmaking of Ignorance*. Stanford University Press.
- **Quine, W. V. O.** (1951). Two dogmas of empiricism. *The Philosophical Review*, 60(1), 20–43.
- **Ramdas, A., Grünwald, P., Vovk, V., & Shafer, G.** (2023). Game-theoretic statistics and safe anytime-valid inference. *Statistical Science*, 38(4), 576–601.
- **Rubin, D. B.** (1976). Inference and missing data. *Biometrika*, 63(3), 581–592.
- **Rudner, R.** (1953). The scientist *qua* scientist makes value judgments. *Philosophy of Science*, 20(1), 1–6.
- **Samuelson, P. A.** (1971). The "fallacy" of maximizing the geometric mean in long sequences of investing or gambling. *Proceedings of the National Academy of Sciences (PNAS)*, 68(10), 2493–2496.
- **Samuelson, P. A.** (1979). Why we should not make mean log of wealth big though years to act are long. *Journal of Banking & Finance*, 3(4), 305–307.
- **Scanlon, T. M.** (1998). *What We Owe to Each Other*. Harvard University Press.
- **Scarf, H.** (1960). The optimality of $(S, s)$ policies in the dynamic inventory problem. In *Mathematical Methods in the Social Sciences* (pp. 196–202). Stanford University Press.
- **Scheffer, M., Bascompte, J., Brock, W. A., Brovkin, V., Carpenter, S. R., Dakos, V., Held, H., van Nes, E. H., Rietkerk, M., & Sugihara, G.** (2009). Early-warning signals for critical transitions. *Nature*, 461(7260), 53–59.
- **Simchi-Levi, D., Schmidt, W., & Wei, Y.** (2014). From superstorms to factory fires: Managing unpredictable supply-chain disruptions. *Harvard Business Review*, 92(1–2), 96–101.
- **Simonsohn, U., Simmons, J. P., & Nelson, L. D.** (2020). Specification curve analysis. *Nature Human Behaviour*, 4(11), 1208–1214.
- **Sniedovich, M.** (2007). The art and science of modeling decision-making under severe uncertainty. *Decision Making in Manufacturing and Services*, 1(1–2), 111–136.
- **Sniedovich, M.** (2012). Black Swans, red herrings, andizarra Info-Gap's No-Man's-Land. *International Transactions in Operational Research*, 19(1–2), 253–281.
- **Sosa, E.** (2007). *A Virtue Epistemology: Apt Belief and Reflective Knowledge* (Vol. I). Oxford University Press.
- **Sosa, E.** (2015). *Judgment and Agency*. Oxford University Press.
- **Steegen, S., Tuerlinckx, F., Gelman, A., & Vanpaemel, W.** (2016). Increasing transparency through a multiverse analysis. *Perspectives on Psychological Science*, 11(5), 702–712.
- **Stevens, S. S.** (1946). On the theory of scales of measurement. *Science*, 103(2684), 677–680.
- **Strathern, M.** (1997). "Improving ratings": Audit in the British university system. *European Review*, 5(3), 305–321.
- **Sutton, R. S., & Barto, A. G.** (2018). *Reinforcement Learning: An Introduction* (2nd ed.). MIT Press.
- **Tian, J., & Pearl, J.** (2000). Probabilities of causation: Bounds and identification. *Annals of Mathematics and Artificial Intelligence*, 28(1–4), 287–313.
- **Van Evera, S.** (1997). *Guide to Methods for Students of Political Science*. Cornell University Press.
- **Ville, J.** (1939). *Étude critique de la notion de collectif*. Gauthier-Villars.
- **Wald, A.** (1943). *A Method of Estimating Plane Vulnerability Based on Damage of Survivors*. Statistical Research Group, Columbia University (CRC 432).
- **Wald, A.** (1945). Statistical decision functions which minimize the maximum risk. *Annals of Mathematics*, 46(2), 265–280.
- **Wang, R., & Ramdas, A.** (2022). False discovery rate control with e-values. *Journal of the Royal Statistical Society: Series B*, 84(3), 822–852.
- **Williams, B.** (1981). *Moral Luck*. Cambridge University Press.
- **Wong, A., Otles, E., Donnelly, J. P., Krumm, A., McCullough, J., DeTroyer-Cooley, O., Pestrue, J., Phillips, M., Konye, J., Penoza, C., Ghous, M., & Singh, K.** (2021). External validation of a widely implemented proprietary sepsis prediction model in hospitalized patients. *JAMA Internal Medicine*, 181(8), 1065–1070.
- **Zheng, J., & Meister, M.** (2025). The unbearable slowness of being: Why do we live at 10 bits/s? *Neuron*, 113(2), 192–204.
