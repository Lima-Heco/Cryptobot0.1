# Cryptobot

| Taches | etat |
| --------- | --------- |
| implementer les pentes  | 100%  |
| implementer l'IHM   | 15%  |
| initialiser le multithread   | 100%  |

# informations

findivp1 et findivp2 etait a l'envers !!

cela posait probleme a findivp2 mais findivp1 a l'aire de s'en accomoder:

          findivp1:     tentatives: 94 reussites: 58

           pourcentage de reussites: 61%

le pourcentage de reussite a stagner entre 61% et 67 % pendant pllus de 4 h !!

# idees:

          J'ai remarquer que les paterns sont plus efficace dans certains contexte. il faut alors que je cree une espece de chef d'orchestre

--------------------------------------------------------------

Reconaissance de paterne qui va rendre propice un autre paterne.

     exemple :
          pique de plus de 250 sur moins de 30 elements va entrainer un terrain propice au findivp1 et findivp2.

Ce proceder va certainement focaliser les situations gagnantes.

-----------------------------------------------------------------------------------------

selection d'un seul paterne par bot via commande.

          favorisera la creation de plusieurs bots.

-----------------------------------------------------------------------------------------

amelioration du bot pour lui apprendre a gerer lui meme sa frequence d'enregistrement.

          permettera la determination de differents bots et de leurs utilitees.

-----------------------------------------------------------------------------------------

NETTOYAGE CODE !!!!

          rendre le code plus lisible ameliorera drastiquement sa rapidite d'evolution.

# comptrendu N1: 
# FIN du premier test de BFC_1 Resultat: CORRECTE


cryptobot nessecite comme prevu une amelioration de son algorythme de vue ainsi que des strategies plus elaboree.

voici le detail des informations trouvees sur une session de 3h:

        comptrendu des investissements : 


      findv1:     tentatives: 3 reussites: 1

           pourcentage de reussites: 33%


      findv2:     tentatives: 0 reussites: 0

           pourcentage de reussites: 0%


      findivp1:     tentatives: 8 reussites: 5

           pourcentage de reussites: 62%


      findp1:     tentatives: 6 reussites: 4

           pourcentage de reussites: 66%

pour les deux derniers, le probleme reside dans le moment de leurs activation et des detailles du reste de l'environnement. (fonctionne tres bien sur de piques de marcher significatif)

pour les deux premiers, je soupsonne d'avantage la chance. (le premier est tres mauvais et dois etre retirer)

je soupsonne le deuxiemme d'etre masquer par le premier (son champs d'activation etant plus large que le deuxiemme). 

FIN du premier test de BFC_1 Resultat: CORRECTE

prologation de 15 minutes:

        comptrendu des investissements : 


      findv1:     tentatives: 3 reussites: 1

           pourcentage de reussites: 33%


      findv2:     tentatives: 0 reussites: 0

           pourcentage de reussites: 0%


      findivp1:     tentatives: 9 reussites: 6

           pourcentage de reussites: 66%


      findp1:     tentatives: 8 reussites: 4

           pourcentage de reussites: 50%
