"""This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at http://mozilla.org/MPL/2.0/.
Copyright (c) 2023 Rafael de Conde Reis. All rights reserved."""

# 1° coluna = alpha, 2° coluna = ai, 3° coluna = di, 4° coluna = theta
# joint é uma matriz para cada linha da tabela DH onde r signigica uma junta rotativa e p uma junta prismática

from library import *

# tabela_DH =([['alpha_1','a_1','d_1','theta_1'],
#             ['alpha_2','a_2','d_2','theta_2']])
# joints = (['r', 'r'])

tabela_DH = symbolization(tabela_DH, joints)

lista_itens = direct_kinematic(tabela_DH, joints)

print(lista_itens[1])
#finish = dt.datetime.now()
print(lista_itens[2])
print(lista_itens[2] - lista_itens[1])
# transformar esse display no que gera a imagem
matrix_dh = lista_itens[0][-1]
latex_equation = s.latex(matrix_dh)
