# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
# Copyright (c) 2023 Rafael de Conde Reis. All rights reserved.

import sympy as s
from sympy.vector import *
import datetime as dt

def is_number(s):
    try:
        float(s)
        return True
    except ValueError:
        return False




def symbolization(x,y):
    t = s.Symbol('t')
    for i in range(0,(len(x))):
        for j in range(0,(len(x[i]))):
            if is_number(x[i][j]):
                aux1 = x[i][j]
                aux2 = float(x[i][j])
                x[i][j] = s.Symbol(x[i][j]).subs({aux1:aux2})
            else:
                if y[i]== 'r' and j==3:
                    x[i][j] = s.Function(x[i][j])(t)
                elif y[i]== 'p' and j==2:
                    x[i][j] = s.Function(x[i][j])(t)
                else:
                    x[i][j] = s.Symbol(x[i][j])
    return x


def delta_t(x):
    return x[1][len(x[1])-1] - x[0][len(x[0])-1]

def calc_zs(J_aux):# the J_aux is assumed to not be changed after it's calculation
    k = s.Matrix([[0],[0],[1]])
    func_aux = J_aux
    #print(func_aux)
    
    z = []
    for i in range(0,len(func_aux)):
        z.append(func_aux[i][0:3,0:3]*k)
       # print("i=",i)
    return z


def calc_zs(J_aux,i):# the J_aux is assumed to not be changed after it's calculation
    k = s.Matrix([[0],[0],[1]])
    func_aux = J_aux
    
    z = func_aux[i][0:3,0:3]*k
    
    return z




def calcula_diferenca_Os(J_aux):# the J_aux is assumed to not be changed after it's calculation
    func_aux = J_aux
    #print(func_aux) 
    
    diferenca = []
    for i in range(1,len(func_aux)):
        diferenca.append((func_aux[-1][0:3,3]-func_aux[i-1][0:3,3]))
        #print("j=",i)
    
    return diferenca

def calcula_diferenca_Os(J_aux,i):# the J_aux is assumed to not be changed after it's calculation
    func_aux = J_aux
    
    
    diferenca = (func_aux[-1][0:3,3]-func_aux[i-1][0:3,3])
    
    
    return diferenca





def cross_product(z,diferenca):
    from sympy.vector import CoordSys3D, matrix_to_vector
    N = CoordSys3D('N')
    z_vector = []
    for i in range(0,len(z)):
        z_vector.append(matrix_to_vector(z[i],N))

    diferenca_vector = []
    for i in range(0,len(diferenca)):
        diferenca_vector.append(matrix_to_vector(diferenca[i],N))
    #len(diferenca_vector)
    J_v_aux = []
    for i in range(0,len(diferenca_vector)):
        J_v_aux.append(z_vector[i]^diferenca_vector[i])
        #print("k=",i)
        
    for i in range(0,len(J_v_aux)):
        J_v_aux[i] = J_v_aux[i].to_matrix(N)
    
    return J_v_aux


def cross_product(J_aux,i):
    from sympy.vector import CoordSys3D, matrix_to_vector
    N = CoordSys3D('N')
    z_vector = matrix_to_vector(calc_zs(J_aux,i),N)

    diferenca_vector = matrix_to_vector(calcula_diferenca_Os(J_aux,i),N)
    
    J_v_aux = z_vector^diferenca_vector
        
    J_v_aux = J_v_aux.to_matrix(N)
    
    return J_v_aux




def direct_kinematic(tabela_DH,joints):
    # calculo dos Ai_i-1

    
    H_n_0= s.Matrix([[ 1, 0, 0, 0],
                  [ 0, 1, 0, 0],
                  [ 0, 0, 1, 0],
                  [ 0, 0, 0, 1]])
    
    iteration_time_marks = [[],[]]
    multiplications_time_marks = [[],[]]
    simplifications_time_marks = [[],[]]
    Ais = []
    J_aux = []
    k = s.Matrix([[0],[0],[1]])
    begin = dt.datetime.now()
    print('começo da multiplicação das matrizes em '+str(begin))
    for i in range(0,(len(tabela_DH))):
        iteration_time_marks[0].append(dt.datetime.now())
        print('-----------------------------------------------------------------------------------------------')
        print('inicio da iteração N° '+str(i+1)+': '+str(iteration_time_marks[0][len(iteration_time_marks[0])-1])+'\n')
    
        
        time_aux = dt.datetime.now()
        Ais.append(s.Matrix([[s.cos(tabela_DH[i][3]), -s.sin(tabela_DH[i][3])*s.cos(tabela_DH[i][0]), s.sin(tabela_DH[i][3])*s.sin(tabela_DH[i][0]), tabela_DH[i][1]*s.cos(tabela_DH[i][3])],                
                       [s.sin(tabela_DH[i][3]), s.cos(tabela_DH[i][3])*s.cos(tabela_DH[i][0]), -s.cos(tabela_DH[i][3])*s.sin(tabela_DH[i][0]), tabela_DH[i][1]*s.sin(tabela_DH[i][3])],            
                       [0 , s.sin(tabela_DH[i][0]) , s.cos(tabela_DH[i][0]), tabela_DH[i][2]],
                       [0, 0, 0, 1]]))
        print('\ttempo de criação da matriz da iteração N° '+str(i+1)+': '+str(dt.datetime.now()-time_aux)+'\n')
        
        
        multiplications_time_marks[0].append(dt.datetime.now())
        
        H_n_0 = H_n_0*Ais[len(Ais)-1]
        J_aux.append(H_n_0) #adicionar a J_w_aux o resultado da multiplicação de k por H_n_0 sendo que cada elemento
        #de J_w_aux é uma coluna do J_w e depois tem que juntar todas elas em uma mat
        multiplications_time_marks[1].append(dt.datetime.now())
        delta = delta_t(multiplications_time_marks)
        print('\ttempo multiplicação da iteração N° '+str(i+1)+': '+str(delta)+'\n')
            
            
        
        
        
        
        #As.append( z_theta*z_d*x_a*x_alpha   )
        iteration_time_marks[1].append(dt.datetime.now())
        delta = delta_t(iteration_time_marks)
        print('tempo da iteração N° '+str(i+1)+': '+str(delta)+'\n\n')
        
    simplifications_time_marks[0].append(dt.datetime.now())
        
    #H_n_0= s.factor(H_n_0)
    #H_n_0= s.trigsimp(H_n_0)
    
        
    simplifications_time_marks[1].append(dt.datetime.now())
    delta = delta_t(simplifications_time_marks)
    print('-----------------------------------------------------------------------------------------------')
    print('\n\n')
    print('\ttempo simplificação da iteração N° '+str(1)+': '+str(delta))
    
    finish = dt.datetime.now() 
    #H_n_0 = s.simplify(H_n_0)
    #H_n_0 = s.trigsimp(H_n_0)
    #H_n_0
    lista_itens = [J_aux,begin,finish, iteration_time_marks, multiplications_time_marks,simplifications_time_marks]
    return lista_itens

def J_aux_preparation(J_aux):
    neutro = [s.Matrix([[1,0,0,0],
                   [0,1,0,0],
                   [0,0,1,0],
                   [0,0,0,1]])]
    for i in range(0,len(J_aux)):
        neutro.append(J_aux[i])
    
    return neutro

#calculating linear velocity part of the Jacobian to a rotational joint
def calc_J_v_aux_r(J_aux):
    return cross_product(  calc_zs(J_aux)  ,  calcula_diferenca_Os(J_aux)  )


#performs the calculation of the jacobian of a given robot based on it's DH table
def Jacobian_calculation(J_aux,joints):
    Jacobian = s.Matrix([])
    for i in range(1, len(J_aux)):
        if( joints[i-1]=='r' ):
            aux_v = cross_product(J_aux,i)
            aux_r = calc_zs(J_aux,i)
            column_aux = s.Matrix.vstack(aux_v,aux_r)
        elif( joints[i-1]=='p' ):
            aux_v = calc_zs(J_aux,i)
            aux_r = s.Matrix([0,0,0])
            column_aux = s.Matrix.vstack(aux_v,aux_r)
        Jacobian = s.Matrix.hstack(Jacobian,column_aux)
        #print(i)
    return Jacobian

