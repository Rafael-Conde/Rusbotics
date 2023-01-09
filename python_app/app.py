"""This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at http://mozilla.org/MPL/2.0/.
Copyright (c) 2023 Rafael de Conde Reis. All rights reserved."""

# 1° coluna = alpha, 2° coluna = ai, 3° coluna = di, 4° coluna = theta
# joint é uma matriz para cada linha da tabela DH onde r signigica uma junta rotativa e p uma junta prismática
# tabela_DH =([['alpha_1','a_1','d_1','theta_1'],
#             ['alpha_2','a_2','d_2','theta_2']])

from library import *
# from IPython.display import display


# tabela_DH = ([['0', 'a_1', '0', 'theta_1'],
"""This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
Copyright (c) 2023 Rafael de Conde Reis. All rights reserved.
file, You can obtain one at http://mozilla.org/MPL/2.0/."""
#              ['0', 'a_2', '0', 'theta_2']])
#              #['alpha_3','a_3','d_3','theta_3']])#,
#              #['alpha_4','a_4','d_4','theta_4'],
#              #['alpha_5','a_5','d_5','theta_5'],
#              #['alpha_6','a_6','d_6','theta_6']])
# joints = (['r', 'r'])  # , 'r'])  # ,'r','r','r'])

tabela_DH = symbolization(tabela_DH, joints)

lista_itens = direct_kinematic(tabela_DH, joints)

print(lista_itens[1])
#finish = dt.datetime.now()
print(lista_itens[2])
print(lista_itens[2] - lista_itens[1])
# transformar esse display no que gera a imagem
latex_equation = s.latex(s.trigsimp(lista_itens[0][-1]))

# print(type(latex_equation))

# p = s.Plot(s.trigsimp(lista_itens[0]))
# p.saveimage('/tmp/plot.png', format='png')
# J_aux = J_aux_preparation(lista_itens[0])
# Jacobian = Jacobian_calculation(J_aux, joints)
# display(s.trigsimp(Jacobian))
