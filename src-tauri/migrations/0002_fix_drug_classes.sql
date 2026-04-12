-- Migration 0002: Fix drug class encoding in tb_treatment_plans
-- Converts old icode-format drugs arrays (["1430104","1000265",...])
-- to drug class letter arrays (["H","R","Z","E"] or ["H","R"])
--
-- Intensive phase rows contain PZA icode 1000258  → ["H","R","Z","E"]
-- Continuation phase rows do NOT contain 1000258  → ["H","R"]

UPDATE tb_treatment_plans
SET drugs = CASE
    WHEN drugs LIKE '%1000258%' THEN '["H","R","Z","E"]'
    ELSE '["H","R"]'
END
WHERE drugs LIKE '%1430104%'
   OR drugs LIKE '%1000265%'
   OR drugs LIKE '%1000264%'
   OR drugs LIKE '%1600004%'
   OR drugs LIKE '%1000129%'
   OR drugs LIKE '%1000258%';