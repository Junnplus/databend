SELECT FORMAT(12332.123456, 4);
SELECT FORMAT(-12332.123456, 4);
SELECT FORMAT(12332.1, 4);
SELECT FORMAT(12332.2, 0);
SELECT FORMAT(12332.2, -1);
SELECT FORMAT(12332, 2);
SELECT FORMAT(0, 0);
SELECT FORMAT(NULL, 1);
SELECT FORMAT(1, NULL);
SELECT FORMAT(NULL, NULL);
SELECT FORMAT(12332.123456, 4, 'en_US');
SELECT FORMAT(12332.123456, 4, 'zh_CN');
SELECT FORMAT(12332.123456, 4, '');
SELECT FORMAT(12332.123456, 4, NULL);
SELECT FORMAT(100 + 100, 2);
SELECT FORMAT(number, number), number from  numbers(3) order by number;