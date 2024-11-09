# Ideas

Loop through every type of product and find the best combination.

`best combination` = lowest buy price + highest sell price

_Keep in mind that the buy price must appear **before** the sell price._ ✅

Another thing to keep in mind is that only **1 product** may be in posession at a time.

Solution for the aforementioned problem:

After gathering all the `best combination`s, loop through them and check if the purchase of the second product appears after the purchase of the first product and simultaneously before the sale of the first product.
In this case compare the profit between the 2 products and only buy the product with the higher profit.

diagram showcasing the condition (invalid combinations):
```
BUY1(15) BUY1(10) BUY2(1) SELL1(30) SELL2(20) SELL1(20)
```
combinations:
```
1: BUY1(15) SELL1(30)
2: BUY1(10) SELL1(30)
3: BUY2(1) SELL2(20)
```

valid combinations:
```
BUY1 SELL1 BUY2 SELL2
```

## How to form combinations

Loop through the `BUY` lines of the input.

If the line is a `BUY` line, loop through the input from this purchase to the end and search for a `SELL` line with the highest profit.
