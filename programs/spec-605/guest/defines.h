/**************************************************************************
DEFINES.H of ZIB optimizer MCF, SPEC version

Dres. Loebel, Borndoerfer & Weider GbR (LBW)
Churer Zeile 15, 12205 Berlin

Konrad-Zuse-Zentrum fuer Informationstechnik Berlin (ZIB)
Scientific Computing - Optimization
Takustr. 7, 14195 Berlin

This software was developed at ZIB Berlin. Maintenance and revisions 
solely on responsibility of LBW

Copyright (c) 1998-2000 ZIB.           
Copyright (c) 2000-2002 ZIB & Loebel.  
Copyright (c) 2003-2005 Loebel.
Copyright (c) 2006-2010 LBW.
**************************************************************************/
/*  LAST EDIT: Tue May 25 23:46:22 2010 by Loebel (opt0.zib.de)  */
/*  $Id: defines.h,v 1.14 2010/05/25 21:58:44 bzfloebe Exp $  */



#ifndef _DEFINES_H
#define _DEFINES_H

#include <zkvm.h>
// #include <stdio.h>
#ifndef _WIN32
// #include <unistd.h>
#endif
// #include <math.h>
// #include <stdlib.h>
// #include <limits.h>
// #include <string.h>
// #include <errno.h>
// #include <ctype.h>
// #include <assert.h>

#ifdef INTERNAL_TIMING
#include <time.h>
#include <sys/times.h>
#include <sys/time.h>
#endif

#ifdef SPEC
// #  include <stdint.h>
#  if defined(SPEC_WINDOWS) && !defined(SPEC_HAVE_INTTYPES_H)
#   include "win32/inttypes.h"
#  else
// #   include <inttypes.h>
#  endif
/* inttypes.h is just to get PRId64; if it's not present (not C99?), guess */
#  if !defined(PRId64)
#   if defined(SPEC_LP64) || defined(SPEC_ILP64)
#    define PRId64 "ld"
#   else
#    define PRId64 "lld"
#   endif
#  endif
#  define LONG int64_t
#else
#  define LONG long
#endif

#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#include <omp.h>
#endif

#include "prototyp.h"


#define UNBOUNDED        1000000000
#define ZERO             0
#define MAX_ART_COST     (LONG)(100000000L)
#define ARITHMETIC_TYPE "I"
#define NEW_ARC          -3
#define DUMMY_ARC        -2
#define DELETED          -1



#define FIXED           -1
#define BASIC            0
#define AT_LOWER         1
#define AT_UPPER         2
/* #define AT_ZERO           3  NOT ALLOWED FOR THE SPEC VERSION */
#undef AT_ZERO

//#define DEBUG 1
//#define AT_HOME 1

#define UP    1
#define DOWN  0



typedef LONG flow_t;
typedef LONG cost_t;

#define K 4000
#define B  60

#define PUFFER 200

#define ITERATIONS_FOR_SMALL_NET 1000
#define ITERATIONS_FOR_BIG_NET 2000


#ifndef NULL
#define NULL 0
#endif


#ifndef ABS
#define ABS( x ) ( ((x) >= 0) ? ( x ) : -( x ) )
#endif


#ifndef MAX
#define MAX(a,b) (((a) > (b)) ? (a) : (b))
#endif


#ifndef SET_ZERO
#define SET_ZERO( vec, n ) if( vec ) memset( (void *)vec, 0, (size_t)n )
#endif


#ifndef FREE
#define FREE( vec ) if( vec ) free( (void *)vec )
#endif


typedef struct node node_t;
typedef struct node *node_p;

typedef struct arc arc_t;
typedef struct arc *arc_p;

typedef struct basket
{
    arc_t *a;
    cost_t cost;
    cost_t abs_cost;
    LONG number;
} BASKET;


struct node
{
  cost_t potential; 
  int orientation;
  node_p child;
  node_p pred;
  node_p sibling;
  node_p sibling_prev;     
  arc_p basic_arc; 
  arc_p firstout, firstin;
  arc_p arc_tmp;
  flow_t flow;
  LONG depth; 
  int number;
  int time;
};



struct arc
{
  int id;
  cost_t cost;
  node_p tail, head;
  short ident;
  arc_p nextout, nextin;
  flow_t flow;
  cost_t org_cost;
};



typedef struct network
{
  char inputfile[200];
  char clustfile[200];
  LONG n, n_trips;
  LONG max_m, m, m_org, m_impl;
  LONG max_residual_new_m, max_new_m;
  
  LONG primal_unbounded;
  LONG dual_unbounded;
  LONG perturbed;
  LONG feasible;
  LONG eps;
  LONG opt_tol;
  LONG feas_tol;
  LONG pert_val;
  LONG bigM;
  double optcost;  
  cost_t ignore_impl;
  node_p nodes, stop_nodes;
  arc_p arcs, stop_arcs, sorted_arcs;
  arc_p dummy_arcs, stop_dummy; 
  LONG iterations;
  LONG bound_exchanges;
  LONG nr_group, full_groups, max_elems;
} network_t;

typedef struct list_elem
{
  arc_t* arc;
  struct list_elem* next;
}list_elem;

typedef struct double_list_elem
{
  arc_t* arc;
  struct double_list_elem* next, *before;
}double_list_elem;

#endif
