# Systembolaget product scraper

> Written to figure out which product has the best APK (Alcohol per krona)

A scraper for the ~~un~~offical [systembolaget.se](https://www.systembolaget.se/) API.

The scraper fetches all of the pages of the API and then stores the information in a SQLite database, for easy use later on. It also does provide some prebuilt features for easy to use later on:

- Calculates the [Alcohol per krona (APK) (wikipedia article in Swedish)](https://sv.wikipedia.org/wiki/Alkohol_per_krona)
- Resolves the image and gives you back a correct url
- Gives the url to the product page

---

## Top APK

| APK (ml/kr) | Product Name        | Price (kronor) | Alcohol Percentage | Volume (ml) | Category   | Link                                              |
| :---------- | :------------------ | :------------- | :----------------- | :---------- | :--------- | :------------------------------------------------ |
| 2.29        | Bello               | 59.0           | 13.5               | 1000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/5868211/ |
| 2.2         | Polly Engraçado     | 59.0           | 13.0               | 1000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/5963201/ |
| 2.19        | Appassinero         | 199.0          | 14.5               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/7908808/ |
| 2.16        | Arboga 10,2         | 23.6           | 10.2               | 500.0       | Ljus lager | https://www.systembolaget.se/produkt/ol/1139212/  |
| 2.11        | Candidato Ecologico | 185.0          | 13.0               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/2233508/ |
| 2.11        | Castillo de Molina  | 199.0          | 14.0               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/255808/  |
| 2.09        | Sofiero Original    | 17.9           | 7.5                | 500.0       | Ljus lager | https://www.systembolaget.se/produkt/ol/127312/   |
| 2.09        | Montevalle Rosso    | 179.0          | 12.5               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/7115808/ |
| 2.07        | Sju komma tvåan     | 17.4           | 7.2                | 500.0       | Ljus lager | https://www.systembolaget.se/produkt/ol/156812/   |
| 2.06        | Il Capolavoro       | 189.0          | 13.0               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/7486208/ |
| 2.06        | Diamond Cellars     | 189.0          | 13.0               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/7974408/ |
| 2.05        | The Missing Piece   | 99.0           | 13.5               | 1500.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/242007/  |
| 2.04        | Farmers Market      | 199.0          | 13.5               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/2007708/ |
| 2.04        | The Loyal Family    | 199.0          | 13.5               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/7042408/ |
| 2.04        | Casteloro           | 199.0          | 13.5               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/7511008/ |
| 2.04        | Vina Nela           | 199.0          | 13.5               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/7561608/ |
| 2.04        | Cappo               | 199.0          | 13.5               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/7161708/ |
| 2.04        | Don Simon           | 199.0          | 13.5               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/7179708/ |
| 2.04        | Pata Negra          | 199.0          | 13.5               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/7268108/ |
| 2.01        | Mira                | 209.0          | 14.0               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/251908/  |
| 2.01        | Dandy               | 209.0          | 14.0               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/5447008/ |
| 2.01        | Pripps Blå Extra    | 16.9           | 6.8                | 500.0       | Ljus lager | https://www.systembolaget.se/produkt/ol/129812/   |
| 1.99        | Castillo de Gredos  | 196.0          | 13.0               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/1279708/ |
| 1.99        | Il Capolavoro       | 219.0          | 14.5               | 3000.0      | Rött vin   | https://www.systembolaget.se/produkt/vin/5328508/ |
| 1.99        | Porta 6             | 49.0           | 13.0               | 750.0       | Rött vin   | https://www.systembolaget.se/produkt/vin/2007511/ |
