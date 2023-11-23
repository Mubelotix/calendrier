# calendrier

Crate for handling dates in the french Revolutionary calendar.

Will provide precise dates from 1583 to 2999 (years whose equinoxes are known).
The equinoxe dates [were collected and computed](https://www.imcce.fr/newsletter/docs/Equinoxe_printemps_1583_2999.pdf) by the Observatoire de Paris.

## Calendar specification

```text
Article premier.
L’ère des Français compte de la fondation de la république, qui a eu lieu le 22 septembre 1792 de l’ère vulgaire, jour où le soleil est arrivé à l’équinoxe vrai d’automne, en entrant dans le signe de la Balance, à 9 heures 18 minutes 30 secondes du matin pour l’observatoire de Paris.

II.
L’ère vulgaire est abolie pour les usages civils.

III.
Chaque année commence à minuit, avec le jour où tombe l’équinoxe vrai d’automne pour l’observatoire de Paris.

IV.
La première année de la République française a commencé à minuit le 22 septembre 1792, et a fini à minuit, séparant le 21 du 22 septembre 1793.

V.
La seconde année a commencé le 22 septembre 1793 à minuit, l’équinoxe vrai d’automne étant arrivé ce jourlà, pour l’observatoire de Paris, à 3 heures 11 minutes et 38 secondes du soir.

VI.
Le décret qui fixait le commencement de la seconde année au Ier janvier 1793, est rapporté ; tous les actes datés l’an second de la République, passés dans le courant du Ier janvier au 21 septembre inclusivement, sont regardés comme appartenant à la première année de la République.

VII.
L’année est divisée en douze mois égaux, de trente jours chacun : après les douze mois suivent cinq jours pour compléter l’année ordinaire ; ces 5 jours n’appartiennent à aucun mois.

VIII.
Chaque mois est divisé en trois parties égales, de dix jours chacune, qui sont appelées décades.

IX.
Les noms des jours de la décade sont :

Primedi,
Duodi,
Tridi,
Quartidi,
Quintidi,
Sextidi,
Septidi,
Octidi,
Nonidi,
Décadi.

Les noms des mois sont :

Pour l’Automne :
Vendémiaire,
Brumaire,
Frimaire ;

Pour l’Hiver :
Nivôse,
Pluviôse,
Ventôse ;

Pour le Printemps :
Germinal,
Floréal,
Prairial ;

Pour l’Été :
Messidor,
Thermidor,
Fructidor.

Les cinq derniers jours s’appellent les Sansculotides.

X.
L’année ordinaire reçoit un jour de plus, selon que la position de l’équinoxe le comporte, afin de maintenir la coïncidence de l’année civile avec les mouvemens célestes. Ce jour, appelé jour de la Révolution, est placé à la fin de l’année et forme le sixième des Sansculotides.
La période de quatre ans, au bout de laquelle cette addition d’un jour est ordinairement nécessaire, est appelée la Franciade, en mémoire de la révolution qui, après quatre ans d’efforts, a conduit la France au gouvernement républicain. La quatrième année de la Franciade est appelée Sextile.

XI.
Le jour, de minuit à minuit, est divisé en 10 parties ou heures, chaque partie en dix autres ; ainsi de suite jusqu’à la plus petite portion commensurable de la durée. La 100e partie de l’heure est appelée minute décimale ; la 100e partie de la minute est appelée seconde décimale. Cet article ne sera de rigueur pour les actes publics, qu’à compter du Ier Vendémiaire, l’an trois de la République.

XII.
Le comité d’instruction publique est chargé de faire imprimer en différens formats le nouveau calendrier, avec une instruction simple pour en expliquer les principes et l’usage.

XIII.
Le calendrier, ainsi que l’instruction, seront envoyés aux corps administratifs, aux municipalités, aux tribunaux, aux juges-de-paix et à tous les officiers publics, aux armées, aux sociétés populaires et à tous les colléges et écoles. Le conseil exécutif provisoire le fera passer aux ministres, consuls et autres agens de France dans les pays étrangers.

XIV.
Tous les actes publics seront datés suivant la nouvelle organisation de l’année.

XV.
Les professeurs, les instituteurs et institutrices, les pères et mères de famille, et tous ceux qui dirigent l’éducation des enfans, s’empresseront à leur expliquer le nouveau calendrier, conformément à l’instruction qui y est annexée.

XVI.
Tous les quatre ans, ou toutes les franciades, au jour de la révolution, il sera célébré des jeux républicains, en mémoire de la révolution française.
```
