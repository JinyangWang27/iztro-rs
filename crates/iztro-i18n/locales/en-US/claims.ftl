# Classical rule-engine claim labels: domains, themes, polarity, claim short text,
# and evidence templates. Keys are derived from the stable enum identities in
# `iztro::rules::classical` (kebab-case), never from Chinese display text.
#
# Claim short-text keys are the rule's `claim_key` with dots mapped to hyphens
# (dots are not valid in Fluent message identifiers).

# -- Claim domains (领域) ---------------------------------------------------
claim-domain-life = Life
claim-domain-body = Body
claim-domain-temperament = Temperament
claim-domain-career = Career
claim-domain-wealth = Wealth
claim-domain-migration = Migration
claim-domain-relationship = Relationship
claim-domain-marriage = Marriage
claim-domain-children = Children
claim-domain-parents = Parents
claim-domain-siblings = Siblings
claim-domain-friends = Friends
claim-domain-property = Property
claim-domain-health = Health
claim-domain-fortune = Fortune
claim-domain-timing = Timing

# -- Claim themes (主题) ----------------------------------------------------
claim-theme-restless-movement = Restless movement
claim-theme-remote-development = Remote development
claim-theme-instability = Instability
claim-theme-obstruction = Obstruction
claim-theme-nobleman-support = Nobleman support
claim-theme-lack-of-support = Lack of support
claim-theme-authority = Authority
claim-theme-responsibility = Responsibility
claim-theme-work-pressure = Work pressure
claim-theme-career-achievement = Career achievement
claim-theme-wealth-accumulation = Wealth accumulation
claim-theme-financial-volatility = Financial volatility
claim-theme-financial-loss = Financial loss
claim-theme-asset-building = Asset building
claim-theme-reputation = Reputation
claim-theme-literary-talent = Literary talent
claim-theme-communication = Communication
claim-theme-harmony = Harmony
claim-theme-conflict = Conflict
claim-theme-separation = Separation
claim-theme-loneliness = Loneliness
claim-theme-stability = Stability
claim-theme-ambition = Ambition
claim-theme-impulsiveness = Impulsiveness
claim-theme-anxiety = Anxiety
claim-theme-vitality = Vitality
claim-theme-illness-risk = Illness risk
claim-theme-injury-risk = Injury risk
claim-theme-blessing = Blessing
claim-theme-constraint = Constraint
claim-theme-damage = Damage
claim-theme-hidden-risk = Hidden risk

# -- Claim polarity (吉凶) --------------------------------------------------
claim-polarity-positive = Positive
claim-polarity-negative = Negative
claim-polarity-mixed = Mixed
claim-polarity-mixed-positive = Mixed-positive
claim-polarity-mixed-negative = Mixed-negative

# -- Claim short text -------------------------------------------------------
claim-migration-tian-ma-void-restless-movement = Tian Ma is affected by a void condition, indicating restless movement or an unsettled life rhythm.
claim-life-yang-tuo-clamp-life-constraint-damage = Qing Yang and Tuo Luo clamp the Life palace, indicating pressure, constraint, or damage.
claim-life-chang-qu-clamp-life-literary-reputation = Wen Chang and Wen Qu clamp the Life palace, indicating literary talent, reputation, or distinction.
claim-life-ri-yue-fan-bei-hardship-pressure = The Sun and Moon are both in weak brightness states, indicating toil, pressure, or reduced clarity.
claim-relationship-tan-ju-hai-zi-water-romance = Tan Lang is placed in Hai or Zi, indicating a water-romance peach-blossom pattern.
claim-relationship-xing-yu-tan-lang-romance-with-penalty = Tan Lang meets a punishment star, indicating romance entangled with conflict or penalty.

# -- Evidence templates (illustrative; rendered by a future narrative layer) -
claim-evidence-star-clamps-palace = { $star } clamps the palace at { $target } from { $clamp }
claim-evidence-affected-by-void = { $star } is affected by a void ({ $void }) at { $branch }
claim-evidence-brightness = { $star } is { $brightness } at { $branch }
