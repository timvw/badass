WITH items AS (
    SELECT 'Tim' AS name, 'Van Wassenhove' as surname
    UNION ALL
    SELECT 'Tiebe' AS name, 'Van Wassenhove' as surname
    UNION ALL
    SELECT 'Amber' AS name, 'Van Wassenhove' as surname
    UNION ALL
    SELECT 'Evy' AS name, 'Penninckx' as surname
)
SELECT *
FROM items