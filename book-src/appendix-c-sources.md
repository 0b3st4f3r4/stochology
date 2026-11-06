# Appendix C — Empirical Datasets and Bibliography

## C.1 Sealed Empirical Datasets (`data/*.csv` & `data/SHA256SUMS`)

Every empirical measurement (`[Measured]`) reported in this book is computed directly from the 14 real-world CSV datasets sealed in [`data/SHA256SUMS`](data/SHA256SUMS) and documented in [`data/SOURCES.md`](data/SOURCES.md):

| Dataset File | Rows ($n$) | Provenance & Domain | Chapter Laboratory |
| :--- | :---: | :--- | :---: |
| `data/nsw_experimental.csv` | $445$ | Robert LaLonde (1986) / Dehejia & Wahba (1999) National Supported Work (NSW) randomized job-training trial (`treat`, `re74`, `re75`, `re78`). | Lab 3 (Ch. 44), Lab 6 (Ch. 47) |
| `data/cps_observational.csv` | $15{,}992$ | Current Population Survey (CPS-1) non-experimental comparison cohort paired with NSW (`age`, `educ`, `black`, `hisp`, `married`, `nodegree`, `re74`, `re75`, `re78`). | Lab 3 (Ch. 44), Lab 4 (Ch. 45) |
| `data/bank_calls.csv` | $27{,}716$ | Anonymous Bank Call Center timestamped call arrivals and service durations across 12 months (Brown et al., 2005). | Lab 4 (Ch. 45) |
| `data/rtdsm_routput_vintages.csv` | $47{,}405$ | Federal Reserve Bank of Philadelphia Real-Time Data Set for Macroeconomists (`ROUTPUT`, Croushore & Stark, 2001): quarterly real GDP/GNP across 244 historical vintages ($Q = 241$ evaluated quarters spanning 1965:Q3 to 2026:Q2; vintage releases run 1965:Q4 to 2026:Q3). | Lab 4 (Ch. 45) |
| `data/danish_fire.csv` | $2{,}167$ | Copenhagen Reinsurance Danish fire insurance claims (1980–1990) exceeding $1\text{M}$ DKK (sealed column `Loss`; McNeil 1997). | Lab 5 (Ch. 46) |
| `data/nyc_flights_2013.csv` | $141{,}193$ | U.S. Bureau of Transportation Statistics (BTS) NYC flight departure and arrival delays (Aug–Dec 2013, $141{,}193$ total rows, $n = 138{,}375$ valid delays, $2{,}818$ `NA` cancellations, $n_+ = 52{,}330$ positive-delay tail in Lab 5 (Ch. 46)). | Lab 5 (Ch. 46) |
| `data/nyc_flights_2023.csv` | $176{,}739$ | U.S. BTS NYC flight departure and arrival delays (Aug–Dec 2023 regime comparison, $176{,}739$ total rows, $n = 173{,}566$ valid delays, $3{,}173$ `NA` cancellations, $n_+ = 48{,}602$ positive-delay tail in Lab 5 (Ch. 46)). | Lab 5 (Ch. 46) |
| `data/nyc_flights_2013_h1.csv` | $195{,}583$ | Full H1 (Jan–Jul) 2013 BTS NYC flight operations archive ($195{,}583$ total rows, $n = 188{,}971$ valid delays, $6{,}612$ `NA` cancellations, $n_+ = 80{,}674$ in Lab 5 (Ch. 46)). | Lab 5 (Ch. 46) |
| `data/nyc_flights_2023_h1.csv` | $258{,}613$ | Full H1 (Jan–Jul) 2023 BTS NYC flight operations archive ($258{,}613$ total rows, $n = 249{,}252$ valid delays, $9{,}361$ `NA` cancellations, $n_+ = 92{,}972$ in Lab 5 (Ch. 46)). | Lab 5 (Ch. 46) |
| `data/houston_flights_2011.csv` | $224{,}591$ | U.S. BTS Houston (IAH/HOU) 2011 flight operations archive ($224{,}591$ total rows, $n = 223{,}874$ valid delays, $717$ `NA` cancellations, $n_+ = 106{,}920$ in Lab 5 (Ch. 46)). | Lab 5 (Ch. 46) |
| `data/djia_1885_2023.csv` | $37{,}931$ | Dow Jones Industrial Average daily closing levels ($37{,}931$ trading days yielding $T = 37{,}930$ daily simple-return pairs) from 1885 to 2023 (Stooq / MeasuringWorth). | Lab 6 (Ch. 47) |
| `data/vic_elec.csv` | $52{,}608$ | Australian Energy Market Operator (AEMO) half-hourly electricity demand (`MW`) and temperature in Victoria (2012–2014). | Lab 6 (Ch. 47), Lab 9 (Ch. 50) |
| `data/atlantic_storms.csv` | $20{,}778$ | NOAA NHC HURDAT2 Atlantic tropical cyclone 6-hourly best-track observations (`1975–2021`, $20{,}778$ rows across $693$ storm tracks yielding $n_{\mathrm{transitions}} = 19{,}025$ strict +6h consecutive same-storm transitions). | Lab 7 (Ch. 48), Lab 9 (Ch. 50) |
| `data/toronto_arrests.csv` | $5{,}226$ | Toronto Police Service arrest and release outcomes by demographic and neighborhood records (Fox & Weisberg `carData::Arrests`). | Lab 8 (Ch. 49) |

---

## C.2 Consolidated Bibliography

- **Adorno, T. W.** (1966). *Negative Dialektik*. Suhrkamp. [Brazilian ed.: *Dialética negativa*. Jorge Zahar, 2009.]
- **Agência Nacional de Aviação Civil** (2024). *Dados estatísticos do transporte aéreo: série histórica*. ANAC, Brasília.
- **Alonso, E., Jelley, A., Kanervisto, A., & Pearce, T.** (2024). Diffusion for world modeling: Visual details matter in Atari. *Advances in Neural Information Processing Systems*, 37.
- **Apollo 13 Mission Evaluation Team** (1970). *Apollo 13 Mission Report* (MSC-02680). NASA Manned Spacecraft Center, Houston.
- **Arendt, H.** (1958). *The Human Condition*. University of Chicago Press. [Brazilian ed.: *A condição humana* (13th ed.). Forense Universitária, 2016.]
- **Argall, B. D., Chernova, S., Veloso, M., & Browning, B.** (2009). A survey of robot learning from demonstration. *Robotics and Autonomous Systems*, 57(5), 469–483.
- **Aristotle** (*c.* 335 BCE). *Nicomachean Ethics* (Books II & VI).
- **Ashby, W. R.** (1956). *An Introduction to Cybernetics*. Chapman & Hall.
- **Assran, M., Duquenne, P.-A., Caron, M., Ballas, N., et al.** (2023). Self-supervised learning from images with a joint-embedding predictive architecture. *Proceedings of the IEEE/CVF Conference on Computer Vision and Pattern Recognition*, 15619–15629.
- **Baker, R. D., & McHale, I. G.** (2013). Optimal betting under parameter uncertainty: Improving the Kelly criterion. *Decision Analysis*, 10(3), 189–199.
- **Bardes, A., Garrido, Q., Ponce, J., Chen, X., Rabbat, M., LeCun, Y., Assran, M., & Ballas, N.** (2025). V-JEPA 2: Self-supervised video models enable understanding, prediction and planning. *arXiv:2506.09985*.
- **Baron, J., & Hershey, J. C.** (1988). Outcome bias in decision evaluation. *Journal of Personality and Social Psychology*, 54(4), 569–579.
- **Bateson, G.** (1972/2000). *Steps to an Ecology of Mind: Collected Essays in Anthropology, Psychiatry, Evolution, and Epistemology*. University of Chicago Press.
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
- **Cook, M.** (2004). Universality in elementary cellular automata. *Complex Systems*, 15(1), 1–40.
- **Croushore, D., & Stark, T.** (2001). A real-time data set for macroeconomists. *Journal of Econometrics*, 105(1), 111–130.
- **Desrosières, A.** (1993). *La Politique des grands nombres: Histoire de la raison statistique*. La Découverte.
- **Dimitriadis, T., Gneiting, T., & Jordan, A. I.** (2021). Stable reliability diagrams for probabilistic classifiers. *Proceedings of the National Academy of Sciences (PNAS)*, 118(8), e2016191118.
- **Dorkenwald, S., Matsliah, A., Sterling, A. R., Schlegel, P., et al.** (2024). Neuronal wiring diagram of an adult brain. *Nature*, 634, 124–138.
- **Douglas, H.** (2000). Inductive risk and values in science. *Philosophy of Science*, 67(4), 559–579.
- **Dreyfus, H. L.** (1972). *What Computers Can't Do: A Critique of Artificial Reason*. Harper & Row.
- **Duhem, P.** (1906). *La Théorie physique: son objet et sa structure*. Chevalier & Rivière.
- **Fantl, J., & McGrath, M.** (2009). *Knowledge in an Uncertain World*. Oxford University Press.
- **Fischer, M. J., Lynch, N. A., & Paterson, M. S.** (1985). Impossibility of distributed consensus with one faulty process. *Journal of the ACM*, 32(2), 374–382.
- **Fricker, M.** (2007). *Epistemic Injustice: Power and the Ethics of Knowing*. Oxford University Press.
- **Gardner, M.** (1970). Mathematical games: The fantastic combinations of John Conway's new solitaire game "Life". *Scientific American*, 223(4), 120–123.
- **Georgescu-Roegen, N.** (1971). *The Entropy Law and the Economic Process*. Harvard University Press.
- **Gigerenzer, G., & Hoffrage, U.** (1995). How to improve Bayesian reasoning without instruction: Frequency formats. *Psychological Review*, 102(4), 684–704.
- **Gilboa, I., & Schmeidler, D.** (1989). Maxmin expected utility with non-unique prior. *Journal of Mathematical Economics*, 18(2), 141–153.
- **Girard, J.-Y.** (1987). Linear logic. *Theoretical Computer Science*, 50(1), 1–101.
- **Girard, R.** (1986 [1982]). *The Scapegoat*. Johns Hopkins University Press. [Brazilian ed.: *O bode expiatório*. Paulus, 2004.]
- **Gneiting, T., & Raftery, A. E.** (2007). Strictly proper scoring rules, prediction, and estimation. *Journal of the American Statistical Association*, 102(477), 359–378.
- **Goldwasser, S., Micali, S., & Rackoff, C.** (1985). The knowledge complexity of interactive proof systems. *SIAM Journal on Computing*, 18(1), 186–208.
- **Goodhart, C. A. E.** (1975). Problems of monetary management: The U.K. experience. *Papers in Monetary Economics*, Reserve Bank of Australia.
- **Griffith, A. A.** (1921). The phenomena of rupture and flow in solids. *Philosophical Transactions of the Royal Society of London. Series A*, 221, 163–198.
- **Ha, D., & Schmidhuber, J.** (2018). Recurrent world models facilitate policy evolution. *Advances in Neural Information Processing Systems*, 31, 2450–2462.
- **Habermas, J.** (1984 [1981]). *The Theory of Communicative Action* (Vol. 1). Beacon Press. [Brazilian ed.: *Teoria do agir comunicativo* (Vol. 1). WMF Martins Fontes, 2012.]
- **Hacking, I.** (2002). *Historical Ontology*. Harvard University Press.
- **Hafner, D., Lillicrap, T., Fischer, I., Villegas, R., Ha, D., Lee, H., & Davidson, J.** (2019). Learning latent dynamics for planning from pixels. *Proceedings of the 36th International Conference on Machine Learning*, 2555–2565.
- **Hafner, D., Pasukonis, J., Ba, J., & Lillicrap, T.** (2023). Mastering diverse domains through world models. *arXiv:2301.04104*.
- **Hansson, S. O.** (2013). *The Ethics of Risk: Ethical Analysis in an Uncertain World*. Palgrave Macmillan.
- **Harding, S.** (1991). *Whose Science? Whose Knowledge? Thinking from Women's Lives*. Cornell University Press.
- **Hill, B. M.** (1975). A simple general approach to inference about the tail of a distribution. *The Annals of Statistics*, 3(5), 1163–1174.
- **Hippocrates** (*c.* 400 BCE). *On Ancient Medicine* (*De Vetere Medicina*) & *Prognostic*.
- **Holling, C. S.** (1973). Resilience and stability of ecological systems. *Annual Review of Ecology and Systematics*, 4, 1–23.
- **Holling, C. S., & Meffe, G. K.** (1996). Command and control and the pathology of natural resource management. *Conservation Biology*, 10(2), 328–337.
- **Holmström, B.** (1979). Moral hazard and observability. *The Bell Journal of Economics*, 10(1), 74–91.
- **Holmström, B., & Milgrom, P.** (1991). Multitask principal–agent analyses: Incentive contracts, asset ownership, and job design. *Journal of Law, Economics, & Organization*, 7, 24–52.
- **Hopfield, J. J.** (1982). Neural networks and physical systems with emergent collective computational abilities. *Proceedings of the National Academy of Sciences*, 79(8), 2554–2558.
- **Hopfield, J. J.** (1974). Kinetic proofreading: A new mechanism for reducing errors in biosynthetic processes requiring high specificity. *Proceedings of the National Academy of Sciences (PNAS)*, 71(10), 4135–4139.
- **Horvitz, D. G., & Thompson, D. J.** (1952). A generalization of sampling without replacement from a finite universe. *Journal of the American Statistical Association*, 47(260), 663–685.
- **Hudgens, M. G., & Halloran, M. E.** (2008). Toward causal inference with interference. *Journal of the American Statistical Association*, 103(482), 832–842.
- **Hutter, M.** (2005). *Universal Artificial Intelligence: Sequential Decisions Based on Algorithmic Probability*. Springer, Berlin.
- **Imbens, G. W., & Manski, C. F.** (2004). Confidence intervals for partially identified parameters. *Econometrica*, 72(6), 1845–1857.
- **Innes, R. D.** (1990). Limited liability and incentive contracting with ex-ante action choices. *Journal of Economic Theory*, 52(1), 45–67.
- **Ioannidis, J. P. A.** (2005). Why most published research findings are false. *PLoS Medicine*, 2(8), e124.
- **Itô, K.** (1951). On a formula concerning stochastic differentials. *Nagoya Mathematical Journal*, 3, 55–65.
- **Jensen, J. L. W. V.** (1906). Sur les fonctions convexes et les inégalités entre les valeurs moyennes. *Acta Mathematica*, 30(1), 175–193.
- **Kant, I.** (1996 [1785]). *Groundwork of the Metaphysics of Morals*. Cambridge University Press. [Portuguese ed.: *Fundamentação da metafísica dos costumes*. Edições 70, 2007.]
- **Kelley, B. N., et al.** (2021). A distributed simulation-to-flight framework to support investigating trust and trustworthiness in multi-agent systems. *AIAA SciTech 2021 Forum* (NASA NTRS 20205010634).
- **Kelly, J. L., Jr.** (1956). A new interpretation of information rate. *Bell System Technical Journal*, 35(4), 917–926.
- **Kelly, K.** (2010). *What Technology Wants*. Viking Press.
- **Kennedy, M. C., & O'Hagan, A.** (2001). Bayesian calibration of computer models. *Journal of the Royal Statistical Society: Series B*, 63(3), 425–464.
- **Kermack, W. O., & McKendrick, A. G.** (1927). A contribution to the mathematical theory of epidemics. *Proceedings of the Royal Society of London. Series A*, 115(772), 700–721.
- **Keynes, J. M.** (1921). *A Treatise on Probability*. Macmillan.
- **Kimball, A. W.** (1957). Errors of the third kind in statistical consulting. *Journal of the American Statistical Association*, 52(278), 133–142.
- **Kingman, J. F. C.** (1961). The single server queue in heavy traffic. *Mathematical Proceedings of the Cambridge Philosophical Society*, 57(4), 902–904.
- **Kish, L.** (1965). *Survey Sampling*. John Wiley & Sons.
- **Kleinberg, J., & Raghavan, M.** (2020). How do classifiers induce agents to invest effort strategically? *ACM Transactions on Economics and Computation*, 8(4), 1–23.
- **Krantz, D. H., Luce, R. D., Suppes, P., & Tversky, A.** (1971). *Foundations of Measurement* (Vol. 1: Additive and Polynomial Representations). Academic Press.
- **Lakatos, I.** (1970). Falsification and the methodology of scientific research programmes. In I. Lakatos & A. Musgrave (Eds.), *Criticism and the Growth of Knowledge* (pp. 91–196). Cambridge University Press.
- **LaLonde, R. J.** (1986). Evaluating the econometric evaluations of training programs with experimental data. *The American Economic Review*, 76(4), 604–620.
- **LeCun, Y.** (2022). A path towards autonomous machine intelligence. *OpenReview preprint* (version 0.9.2).
- **Lee, D. S.** (2009). Training, wages, and sample selection: Estimating sharp bounds on treatment effects. *Review of Economic Studies*, 76(3), 1071–1102.
- **LeMoigne, J., et al.** (2025). NASA Earth System Digital Twins (ESDT) for a sustainable future. *AMS Annual Meeting* (NASA NTRS 20250000171).
- **Lighthill, J.** (1973). *Artificial Intelligence: A General Survey*. Science Research Council, London.
- **Lindley, D. V.** (1985). *Making Decisions* (2nd ed.). John Wiley & Sons.
- **Lions, J.-L.** (1996). *Ariane 5 Flight 501 Failure: Report by the Inquiry Board*. European Space Agency, Paris.
- **List, C., & Pettit, P.** (2011). *Group Agency: The Possibility, Design, and Status of Corporate Agents*. Oxford University Press.
- **Longino, H. E.** (1990). *Science as Social Knowledge: Values and Objectivity in Scientific Inquiry*. Princeton University Press.
- **Lorenz, E. N.** (1963). Deterministic nonperiodic flow. *Journal of the Atmospheric Sciences*, 20(2), 130–141.
- **Lucas, R. E., Jr.** (1976). Econometric policy evaluation: A critique. *Carnegie-Rochester Conference Series on Public Policy*, 1, 19–46.
- **Lum, K., Price, M. E., & Banks, D.** (2013). Applications of multiple systems estimation in human rights research. *The American Statistician*, 67(4), 191–200.
- **MacKenzie, D.** (2006). *An Engine, Not a Camera: How Financial Models Shape Markets*. MIT Press.
- **Manheim, D., & Garrabrant, S.** (2018). Categorizing variants of Goodhart's Law. *arXiv:1803.04585*.
- **Manski, C. F.** (1990). Nonparametric bounds on treatment effects. *The American Economic Review*, 80(2), 319–323.
- **Manski, C. F.** (2003). *Partial Identification of Probability Distributions*. Springer.
- **Mayo, D. G.** (2018). *Statistical Inference as Severe Testing: How to Get Beyond the Statistics Wars*. Cambridge University Press.
- **McEwen, B. S.** (1998). Protective and damaging effects of stress mediators. *New England Journal of Medicine*, 338(3), 171–179.
- **Meng, X.-L.** (2018). Statistical paradoxes in big data: Law of large populations, big data paradox, and the 2016 US presidential election. *The Annals of Applied Statistics*, 12(2), 685–726.
- **Minsky, M.** (1988). *The Society of Mind*. Simon & Schuster.
- **Minsky, M., & Papert, S.** (1969). *Perceptrons: An Introduction to Computational Geometry*. MIT Press. [1988 edition with epilogue.]
- **Mordvintsev, A., Randazzo, E., Niklasson, E., & Levin, M.** (2020). Growing neural cellular automata. *Distill*.
- **Morris, M. R., Sohl-Dickstein, J., Fiedel, N., Warkentin, T., Dafoe, A., Faust, A., Farabet, C., & Legg, S.** (2023). Levels of AGI: Operationalizing progress on the path to AGI. *arXiv:2311.02462*.
- **Motter, A. E., & Lai, Y.-C.** (2002). Cascade-based attacks on complex networks. *Physical Review E*, 66(6), 065102.
- **Moulton, B. R.** (1986). Random group effects and the precision of regression estimates. *Journal of Econometrics*, 32(3), 385–397.
- **Nagel, T.** (1979). Moral luck. In *Mortal Questions* (pp. 24–38). Cambridge University Press.
- **NASA** (2022). *Lunar Regolith: Small Scale Robotic Site Preparation and Geotechnical Experiments with Scoops*. NASA Technical Reports Server (NTRS 20220008834).
- **Ostrom, E.** (1990). *Governing the Commons: The Evolution of Institutions for Collective Action*. Cambridge University Press.
- **Paris, P. C., Gomez, M. P., & Anderson, W. E.** (1961). A rational analytic theory of fatigue. *The Trend in Engineering*, 13(1), 9–14.
- **Peirce, C. S.** (1899/1931). First rule of reason (*Collected Papers*, CP 1.135). Harvard University Press.
- **Perdomo, J., Zrnic, T., Mendler-Dünner, C., & Hardt, M.** (2020). Performative prediction. *Proceedings of the 37th International Conference on Machine Learning (ICML)*, PMLR 119, 7599–7609.
- **Peters, J., Bühlmann, P., & Meinshausen, N.** (2016). Causal inference by using invariant prediction: Identification and confidence intervals. *Journal of the Royal Statistical Society: Series B*, 78(5), 947–1012.
- **Peters, O.** (2019). The ergodicity problem in economics. *Nature Physics*, 15(12), 1216–1221.
- **Peterson, A. V., Jr.** (1976). Bounds for a joint distribution function with fixed sub-distribution functions: Application to competing risks. *Proceedings of the National Academy of Sciences (PNAS)*, 73(1), 11–13.
- **Plato** (*c.* 360 BCE). *Philebus* (55e–56c, Loeb Classical Library Vol. 8).
- **Plutarch** (*c.* 100 CE). *De communibus notitiis adversus Stoicos* (1070f–1071e, Loeb Classical Library Vol. 13, Part 2).
- **Popper, K. R.** (1959). *The Logic of Scientific Discovery*. Hutchinson.
- **Porter, T. M.** (1995). *Trust in Numbers: The Pursuit of Objectivity in Science and Public Life*. Princeton University Press.
- **Proctor, R. N., & Schiebinger, L.** (Eds.). (2008). *Agnotology: The Making and Unmaking of Ignorance*. Stanford University Press.
- **Quine, W. V. O.** (1951). Two dogmas of empiricism. *The Philosophical Review*, 60(1), 20–43.
- **Ramdas, A., Grünwald, P., Vovk, V., & Shafer, G.** (2023). Game-theoretic statistics and safe anytime-valid inference. *Statistical Science*, 38(4), 576–601.
- **Rice, H. G.** (1953). Classes of recursively enumerable sets and their decision problems. *Transactions of the American Mathematical Society*, 74(2), 358–366.
- **Rosenblatt, F.** (1958). The perceptron: A probabilistic model for information storage and organization in the brain. *Psychological Review*, 65(6), 386–408.
- **Roy, A. D.** (1952). Safety first and the holding of assets. *Econometrica*, 20(3), 431–449.
- **Rubin, D. B.** (1976). Inference and missing data. *Biometrika*, 63(3), 581–592.
- **Rudner, R.** (1953). The scientist *qua* scientist makes value judgments. *Philosophy of Science*, 20(1), 1–6.
- **Rumelhart, D. E., Hinton, G. E., & Williams, R. J.** (1986). Learning representations by back-propagating errors. *Nature*, 323, 533–536.
- **Samuelson, P. A.** (1971). The "fallacy" of maximizing the geometric mean in long sequences of investing or gambling. *Proceedings of the National Academy of Sciences (PNAS)*, 68(10), 2493–2496.
- **Samuelson, P. A.** (1979). Why we should not make mean log of wealth big though years to act are long. *Journal of Banking & Finance*, 3(4), 305–307.
- **Savage, L. J.** (1951). The theory of statistical decision. *Journal of the American Statistical Association*, 46(253), 55–67.
- **Savage, L. J.** (1954). *The Foundations of Statistics*. John Wiley & Sons.
- **Scanlon, T. M.** (1998). *What We Owe to Each Other*. Harvard University Press.
- **Scarf, H.** (1960). The optimality of $(S, s)$ policies in the dynamic inventory problem. In *Mathematical Methods in the Social Sciences* (pp. 196–202). Stanford University Press.
- **Scheffer, M., Bascompte, J., Brock, W. A., Brovkin, V., Carpenter, S. R., Dakos, V., Held, H., van Nes, E. H., Rietkerk, M., & Sugihara, G.** (2009). Early-warning signals for critical transitions. *Nature*, 461(7260), 53–59.
- **Schlegel, P., Yin, Y., Bates, A. S., et al.** (2024). Whole-brain annotation and multi-connectome cell typing of Drosophila. *Nature*, 634, 139–152.
- **Shannon, C. E.** (1949). Communication theory of secrecy systems. *Bell System Technical Journal*, 28(4), 656–715.
- **Shiu, P. K., et al.** (2024). A Drosophila computational brain model reveals sensorimotor processing. *Nature*, 634 (article s41586-024-07763-9).
- **Silver, D., Huang, A., Maddison, C. J., Guez, A., et al.** (2016). Mastering the game of Go with deep neural networks and tree search. *Nature*, 529, 484–489.
- **Simchi-Levi, D., Schmidt, W., & Wei, Y.** (2014). From superstorms to factory fires: Managing unpredictable supply-chain disruptions. *Harvard Business Review*, 92(1–2), 96–101.
- **Simon, H. A.** (1997 [1947]). *Administrative Behavior* (4th ed.). Free Press. [Brazilian ed.: *Comportamento administrativo* (3rd ed.). FGV, 1979.]
- **Simonsohn, U., Simmons, J. P., & Nelson, L. D.** (2020). Specification curve analysis. *Nature Human Behaviour*, 4(11), 1208–1214.
- **Sitzmann, V., Martel, J. N. P., Bergman, A. W., Lindell, D. B., & Wetzstein, G.** (2020). Implicit neural representations with periodic activation functions. *Advances in Neural Information Processing Systems*, 33, 7462–7473.
- **Sniedovich, M.** (2007). The art and science of modeling decision-making under severe uncertainty. *Decision Making in Manufacturing and Services*, 1(1–2), 111–136.
- **Sniedovich, M.** (2012). Black Swans, red herrings, and Info-Gap's No-Man's-Land. *International Transactions in Operational Research*, 19(1–2), 253–281.
- **Sosa, E.** (2007). *A Virtue Epistemology: Apt Belief and Reflective Knowledge* (Vol. I). Oxford University Press.
- **Sosa, E.** (2015). *Judgment and Agency*. Oxford University Press.
- **Spinoza, B.** (2007 [1670]). *Theological-Political Treatise*. Cambridge University Press. [Brazilian ed.: Espinosa, B. de. *Tratado teológico-político*. Martins Fontes, 2003.]
- **Steegen, S., Tuerlinckx, F., Gelman, A., & Vanpaemel, W.** (2016). Increasing transparency through a multiverse analysis. *Perspectives on Psychological Science*, 11(5), 702–712.
- **Stevens, S. S.** (1946). On the theory of scales of measurement. *Science*, 103(2684), 677–680.
- **Stobaeus, I.** (1884). *Ioannis Stobaei Anthologii libri duo priores qui inscribi solent Eclogae physicae et ethicae* (C. Wachsmuth, Ed., Vol. 2). Weidmann.
- **Strathern, M.** (1997). "Improving ratings": Audit in the British university system. *European Review*, 5(3), 305–321.
- **Sutton, R. S., & Barto, A. G.** (2018). *Reinforcement Learning: An Introduction* (2nd ed.). MIT Press.
- **Tian, J., & Pearl, J.** (2000). Probabilities of causation: Bounds and identification. *Annals of Mathematics and Artificial Intelligence*, 28(1–4), 287–313.
- **Turing, A. M.** (1950). Computing machinery and intelligence. *Mind*, 59(236), 433–460.
- **Van Evera, S.** (1997). *Guide to Methods for Students of Political Science*. Cornell University Press.
- **Ville, J.** (1939). *Étude critique de la notion de collectif*. Gauthier-Villars.
- **Waddington, C. H.** (1957). *The Strategy of the Genes: A Discussion of Some Aspects of Theoretical Biology*. George Allen & Unwin.
- **Wald, A.** (1943). *A Method of Estimating Plane Vulnerability Based on Damage of Survivors*. Statistical Research Group, Columbia University (CRC 432).
- **Wald, A.** (1945). Statistical decision functions which minimize the maximum risk. *Annals of Mathematics*, 46(2), 265–280.
- **Walley, P.** (1991). *Statistical Reasoning with Imprecise Probabilities*. Chapman & Hall.
- **Wang, R., & Ramdas, A.** (2022). False discovery rate control with e-values. *Journal of the Royal Statistical Society: Series B*, 84(3), 822–852.
- **Weibull, W.** (1939). A statistical theory of the strength of materials. *Ingeniörsvetenskapsakademiens Handlingar*, 151, 1–45.
- **Weil, S.** (1952 [1949]). *The Need for Roots: Prelude to a Declaration of Duties Towards Mankind*. Routledge & Kegan Paul. [Brazilian ed.: *O enraizamento*. EDUSC, 2001.]
- **Williams, B.** (1981). *Moral Luck*. Cambridge University Press.
- **Wolfram, S.** (2002). *A New Kind of Science*. Wolfram Media, Champaign, IL.
- **Wong, A., Otles, E., Donnelly, J. P., Krumm, A., McCullough, J., DeTroyer-Cooley, O., Pestrue, J., Phillips, M., Konye, J., Penoza, C., Ghous, M., & Singh, K.** (2021). External validation of a widely implemented proprietary sepsis prediction model in hospitalized patients. *JAMA Internal Medicine*, 181(8), 1065–1070.
- **Wurman, P. R., Barrett, S., Kawamoto, K., et al.** (2022). Outracing champion Gran Turismo drivers with deep reinforcement learning. *Nature*, 602, 223–228.
- **Zheng, J., & Meister, M.** (2025). The unbearable slowness of being: Why do we live at 10 bits/s? *Neuron*, 113(2), 192–204.
