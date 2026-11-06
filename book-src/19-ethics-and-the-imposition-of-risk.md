# Ethics and the Imposition of Risk

> Fourth chapter of the philosophical block in Part II. The subject is not what is good, but who may
> impose which risk on whom, and on what authority.

## By the end of this chapter you can

1. State the three classical families of ethical theory and the question each one asks first.
2. Explain why imposing risk on others is a different act from imposing it on yourself.
3. Apply the contractualist test to a decision rule and say what fails it.

## 1. Three Ethical Lineages and the Aggregation Fallacy

Normative ethics provides three distinct frameworks for evaluating actions under uncertainty:

- **Consequentialism** (Bentham, Mill): Evaluates an act exclusively by the value of its aggregated outcomes.
  Its operational danger is the **aggregation fallacy**: maximizing total expected utility licenses the
  imposition of catastrophic, uncompensated ruin on a vulnerable minority if the aggregate sum of minor
  conveniences across the majority is sufficiently large.
- **Deontology** (Kant): Evaluates an act by whether it complies with universal moral duties. Kant's formula
  of humanity mandates treating persons always as ends in themselves, never merely as instruments. It forbids
  sacrificing unconsenting individuals for collective optimization, but struggles to provide operational
  thresholds when all available choices carry non-zero statistical risks.
- **Virtue Ethics** (Aristotle): Evaluates the character, practical wisdom (*phrónēsis*), and internal aim
  (*télos*) of the acting agent. It captures the nuance of situated judgment, but resists codification into
  formal administrative rules that an external auditor can inspect.

Stochology does not treat these three lineages as mutually exclusive options. Instead, it recognizes that any
sustainable decision governance must be constrained simultaneously from all three directions: it must account
for physical consequences, respect non-instrumental deontic rights, and evaluate the internal virtue and
calibration of the decision-maker.

## 2. The Asymmetry of Non-Reciprocal Risk Imposition

Traditional decision theory treats risk symmetrically: an agent faces lottery $L = (p, x; 1-p, y)$ and maximizes
expected utility $\mathbb{E}[u(L)]$. This framing obscures the central ethical reality of institutions:
**deciding to take a risk for oneself is morally distinct from imposing that risk on someone else**.

Non-reciprocal risk imposition involves three structural asymmetries:

1. **Absence of consent**: The individuals who bear the physical or financial downside did not choose the risk;
   they were exposed to it by an external authority.
2. **Epistemic opacity**: The exposed individuals cannot audit the mathematical model or risk assumptions that
   justified their exposure; the model is proprietary or administratively remote.
3. **Decoupled incentives (Moral hazard)**: The upside of the risk (profits, executive bonuses, political expediency)
   accrues to the decision-maker, while the catastrophic downside (displacement, bankruptcy, injury, mortality)
   is externalized onto vulnerable third parties.

When downside is decoupled from upside, standard expected-value calculations become an ideological cover for
unconsenting risk transfer. Sizing a vital margin is not a purely financial exercise; it is the moral duty to
prevent externalizing fatal risk onto non-beneficiary populations.

## 3. Scanlonian Contractualism and the Reasonability Test

T.M. Scanlon (1998) formulated **contractualism**: an act or policy is wrong if its performance could only be
justified by principles that affected persons could **reasonably reject**.

Contractualism evaluates policies from the standpoint of individual affected parties, explicitly forbidding
utilitarian interpersonal aggregation. When auditing an automated or institutional decision rule, the contractualist
test asks:

- What is the worst-case burden imposed on the most vulnerable affected individual?
- Could that individual reasonably reject a principle that permits this exposure, given the alternative policies
  available?
- Does the decision rule provide meaningful justification in terms that the exposed person could recognize as valid?

Under contractualism, a decision rule that sacrifices an innocent or vulnerable party to optimize average
institutional efficiency is categorically rejected.

## 4. Moral Luck and the Residue of Agent-Regret

Bernard Williams (1981) and Thomas Nagel (1979) identified the paradox of **moral luck**: our intuitive judgments
of agents depend heavily on factors outside the agent's control:

- **Resultant luck**: How actions turn out due to stochastic environmental noise (e.g., two drivers take an identical
  reckless turn; one encounters an empty road, the other hits a child).
- **Circumstantial luck**: The situations and crises an agent happens to be thrust into.
- **Constitutive luck**: The traits, temperaments, and capacities an agent is endowed with.

Williams showed that an adroit agent whose competent, well-calibrated action nevertheless causes catastrophic harm
experiences a profound psychological residue: **agent-regret**. The agent did not act with malice or negligence,
yet they cannot walk away as a detached spectator.

In institutional design, moral luck requires two structural safeguards:
1. Decision quality must be evaluated based on ex-ante calibration and adroitness, insulating operators from
   punishment for pure resultant bad luck;
2. Physical harm caused by bad luck must be fully compensated through dedicated collective vital reserves,
   rather than abandoned to the unfortunate victim.

## 5. Performative Prediction and Donald MacKenzie's Performativity

In social and financial systems, models are not passive observers of an invariant reality. Donald MacKenzie (2006)
demonstrated in *An Engine, Not a Camera* that financial models (such as Black–Scholes–Merton) exhibit
**Barnesian performativity**: using the model alters market behavior, forcing reality into closer alignment with
the model's theoretical assumptions until systemic limits trigger liquidity crises.

Juan Perdomo et al. (2020) formalized this dynamics as **performative prediction**. In a static world, data are
drawn from a fixed distribution: $(X, Y) \sim \mathcal{D}$. In a reactive world, deploying decision rule $\theta$
changes the underlying distribution itself:

$$(X, Y) \sim \mathcal{D}(\theta).$$

Predicting crime risk changes police deployment, which changes where arrests occur; credit scoring changes borrower
behavior; automated grading changes student study strategies. When an institution ignores performative feedback,
its models optimize against a ghost distribution, causing the system to induce the very failures it claimed
to forecast.

## Bridge

[Ethics of Imposition and Non-Reciprocity](36-decision-loss-threshold.md) analyzes risk externalization, while
[Moral Luck, Apt Agency, and Institutional Design](49-moral-luck-apt-agency-institutions.md) constructs institutional
gates for liability assignment. In the Stochology library, `governance.rs::from_ex_ante_gate` formalizes the
ex-ante evaluation gate, auditing whether decision models account for non-reciprocal risk imposition, performative
distributional shift, and contractualist consent before action is executed.

[Legitimacy, Accountability, and Institutions](20-legitimacy-and-institutions.md) moves the question
from the single decision to the institution that authorizes it, where legitimacy is not correctness and
contestability is the test that remains.
