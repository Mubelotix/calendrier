# calendrier

Crate for handling dates in the french Revolutionary calendar.

It is able to provide correct dates from 1583 to 2999 (years whose equinoxes are well-known).
This is one of the only and most correct implementations of the entire Internet.
Outside of this range, years will begin to shift by one day every few years.

The equinoxe dates [were collected and computed](https://www.imcce.fr/newsletter/docs/Equinoxe_printemps_1583_2999.pdf) by the Observatoire de Paris.

A time offset of 18 minutes is applied to correct the slow shift of the measure of time since the 18th century.

## Usage

Run `cargo run --example today` to know what day it is.

To use it in a library, here is an example:

```rust
use calendrier::*;

let date = DateTime::from_ymd(1, 1, 1); // Calendar starts on september 22nd, 1792
let date_fmt = date.to_string();
assert_eq!(date_fmt.as_str(), "1 Vendémiaire 1");

let ts = date.timestamp(); // Convert to timestamp, number of seconds since republican epoch
assert_eq!(ts, Timestamp { seconds: 0 }); // Timestamps are encapsulated in a struct so that you don't confuse them with unix timestamps

let ts_unix = ts.to_unix(); // Convert to unix timestamp
assert_eq!(ts_unix, -5594228280);
```

## Calendar specification

> Article premier.
> L’ère des Français compte de la fondation de la république, qui a eu lieu le 22 septembre 1792 de l’ère vulgaire, jour où le soleil est arrivé à l’équinoxe vrai d’automne, en entrant dans le signe de la Balance, à 9 heures 18 minutes 30 secondes du matin pour l’observatoire de Paris.
> 
> II.
> L’ère vulgaire est abolie pour les usages civils.
> 
> III.
> Chaque année commence à minuit, avec le jour où tombe l’équinoxe vrai d’automne pour l’observatoire de Paris.
> 
> IV.
> La première année de la République française a commencé à minuit le 22 septembre 1792, et a fini à minuit, séparant le 21 du 22 septembre 1793.
> 
> V.
> La seconde année a commencé le 22 septembre 1793 à minuit, l’équinoxe vrai d’automne étant arrivé ce jour là, pour l’observatoire de Paris, à 3 heures 11 minutes et 38 secondes du soir.
> 
> VI.
> Le décret qui fixait le commencement de la seconde année au Ier janvier 1793, est rapporté ; tous les actes datés l’an second de la République, passés dans le courant du Ier janvier au 21 septembre inclusivement, sont regardés comme appartenant à la première année de la République.
> 
> VII.
> L’année est divisée en douze mois égaux, de trente jours chacun : après les douze mois suivent cinq jours pour compléter l’année ordinaire ; ces 5 jours n’appartiennent à aucun mois.
> 
> VIII.
> Chaque mois est divisé en trois parties égales, de dix jours chacune, qui sont appelées décades.
> 
> IX.
> Les noms des jours de la décade sont :
> 
> Primedi,
> Duodi,
> Tridi,
> Quartidi,
> Quintidi,
> Sextidi,
> Septidi,
> Octidi,
> Nonidi,
> Décadi.
> 
> Les noms des mois sont :
> 
> Pour l’Automne :
> Vendémiaire,
> Brumaire,
> Frimaire ;
> 
> Pour l’Hiver :
> Nivôse,
> Pluviôse,
> Ventôse ;
> 
> Pour le Printemps :
> Germinal,
> Floréal,
> Prairial ;
> 
> Pour l’Été :
> Messidor,
> Thermidor,
> Fructidor.
> 
> Les cinq derniers jours s’appellent les Sansculotides.
> 
> X.
> L’année ordinaire reçoit un jour de plus, selon que la position de l’équinoxe le comporte, afin de maintenir la coïncidence de l’année civile avec les mouvements célestes. Ce jour, appelé jour de la Révolution, est placé à la fin de l’année et forme le sixième des Sansculotides.
> La période de quatre ans, au bout de laquelle cette addition d’un jour est ordinairement nécessaire, est appelée la Franciade, en mémoire de la révolution qui, après quatre ans d’efforts, a conduit la France au gouvernement républicain. La quatrième année de la Franciade est appelée Sextile.
> 
> XI.
> Le jour, de minuit à minuit, est divisé en 10 parties ou heures, chaque partie en dix autres ; ainsi de suite jusqu’à la plus petite portion commensurable de la durée. La 100e partie de l’heure est appelée minute décimale ; la 100e partie de la minute est appelée seconde décimale. Cet article ne sera de rigueur pour les actes publics, qu’à compter du Ier Vendémiaire, l’an trois de la République.
> 
> XII.
> Le comité d’instruction publique est chargé de faire imprimer en différens formats le nouveau calendrier, avec une instruction simple pour en expliquer les principes et l’usage.
> 
> XIII.
> Le calendrier, ainsi que l’instruction, seront envoyés aux corps administratifs, aux municipalités, aux tribunaux, aux juges-de-paix et à tous les officiers publics, aux armées, aux sociétés populaires et à tous les colléges et écoles. Le conseil exécutif provisoire le fera passer aux ministres, consuls et autres agens de France dans les pays étrangers.
> 
> XIV.
> Tous les actes publics seront datés suivant la nouvelle organisation de l’année.
> 
> XV.
> Les professeurs, les instituteurs et institutrices, les pères et mères de famille, et tous ceux qui dirigent l’éducation des enfans, s’empresseront à leur expliquer le nouveau calendrier, conformément à l’instruction qui y est annexée.
> 
> XVI.
> Tous les quatre ans, ou toutes les franciades, au jour de la révolution, il sera célébré des jeux républicains, en mémoire de la révolution française.

The contradiction between article III and IX is resolved by prioritizing article III, as decided in year 79 by the Commune de Paris.
Indeed, a year isn't exactly 365.25 days long, so Franciades will not always be 365*4+1 days long.

Also as a reminder: Romme's WIP reform was never adopted. It wasn't a good idea anyway. This crate will never implement the reform.
