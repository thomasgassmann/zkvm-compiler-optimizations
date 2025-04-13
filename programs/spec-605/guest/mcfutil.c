/**************************************************************************
MCFUTIL.C of ZIB optimizer MCF, SPEC version

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
/*  LAST EDIT: Tue May 25 23:46:54 2010 by Loebel (opt0.zib.de)  */
/*  $Id: mcfutil.c,v 1.12 2010/05/25 21:58:44 bzfloebe Exp $  */



#include "mcfutil.h"

#ifdef _PROTO_
void refresh_neighbour_lists( network_t *net, LONG (*getPos)(network_t*, LONG) )
#else
void refresh_neighbour_lists( net )
    network_t *net;
#endif
{
    node_t *node;
    arc_t *arc;
    void *stop;
    int i;
        

    node = net->nodes;
    for( stop = (void *)net->stop_nodes; node < (node_t *)stop; node++ )
    {
        node->firstin = (arc_t *)NULL;
        node->firstout = (arc_t *)NULL;
    }


    arc = net->arcs;
    for( i = 0; i < net->m; i++, arc = &net->arcs[getPos(net, i)] )
    {
        arc->nextout = arc->tail->firstout;
        arc->tail->firstout = arc;
        arc->nextin = arc->head->firstin;
        arc->head->firstin = arc;
    }
    
    return;
}


#ifdef _PROTO_
double flow_cost( network_t *net )
#else
double flow_cost( net )
    network_t *net;
#endif
{
    arc_t *arc;
    node_t *node;
    
    LONG fleet = 0;
    int i;
    cost_t operational_cost = 0;
    

    arc = net->arcs;
#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp parallel for
#endif
    for( i = 0; i < net->m ; i++ )
    {
        if( arc[i].ident == AT_UPPER )
            arc[i].flow = (flow_t)1;
        else
            arc[i].flow = (flow_t)0;
    }

    node = net->nodes;

#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp parallel for
#endif
    for( i = 1; i <= net->n; i++) {
        node[i].basic_arc->flow = node[i].flow;
    }

    arc = net->arcs;
#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp parallel for reduction(+ : fleet, operational_cost)
#endif
    for( i = 0; i < net->m; i++ )
    {
        if( arc[i].flow )
        {
            if( !(arc[i].tail->number < 0 && arc[i].head->number > 0) )
            {
                if( !arc[i].tail->number )
                {
                    operational_cost += (arc[i].cost - net->bigM);
                    fleet++;
                }
                else
                    operational_cost += arc[i].cost;
            }
        }

    }
    
    return (double)fleet * (double)net->bigM + (double)operational_cost;
}

static LONG old_group = 0;
static LONG old_Arc = 0;

#ifdef _PROTO_
LONG start()
#else
start()
#endif
{
    old_group = 0;
    old_Arc = 0;

    return 0;
}

#ifdef _PROTO_
LONG getArcPosition(network_t *net, LONG actArc)
#else
int getArcPosition(net, actArc)
network_t *net;
LONG actArc;
#endif
{
  LONG result, akt_group;
  akt_group = actArc % net->nr_group;
  if (akt_group > net->full_groups) {
    result = (actArc / net->nr_group) + (net->full_groups * net->max_elems + (akt_group - net->full_groups) * (net->max_elems -1));
  }
  else {
    result = (actArc / net->nr_group) + (akt_group * net->max_elems);
  }
    return result;
}

#ifdef _PROTO_
LONG getOriginalArcPosition(network_t *net, LONG actArc)
#else
getOriginalArcPosition(net, actArc)
network_t *net;
LONG actArc;
#endif
{
  return actArc;
}





#ifdef _PROTO_
double flow_org_cost( network_t *net )
#else
double flow_org_cost( net )
    network_t *net;
#endif
{
    arc_t *arc;
    node_t *node;
    int i;
    
    LONG fleet = 0;
    cost_t operational_cost = 0;
    
    arc = net->arcs;
#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp parallel for
#endif
    for( i = 0; i < net->m; i++ )
    {
        if( arc[i].ident == AT_UPPER )
            arc[i].flow = (flow_t)1;
        else
            arc[i].flow = (flow_t)0;
    }

    node = net->nodes;
#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp parallel for reduction(+: fleet, operational_cost)
#endif
    for( i = 0; i < net->n; i++ )
        node[i].basic_arc->flow = node[i].flow;
    
    arc = net->arcs;
    for( i = 0; i < net->m; i++ )
    {
        if( arc[i].flow )
        {
            if( !(arc[i].tail->number < 0 && arc[i].head->number > 0) )
            {
                if( !arc[i].tail->number )
                {
                    operational_cost += (arc[i].org_cost - net->bigM);
                    fleet++;
                }
                else
                    operational_cost += arc[i].org_cost;
            }
        }
    }
    
printf("ORG_COST: %f", (double)fleet * (double)net->bigM + (double)operational_cost);
    return (double)fleet * (double)net->bigM + (double)operational_cost;
}










#ifdef _PROTO_
LONG primal_feasible( network_t *net )
#else
LONG primal_feasible( net )
    network_t *net;
#endif
{
    void *stop;
    node_t *node;
    arc_t *dummy = net->dummy_arcs;
    arc_t *stop_dummy = net->stop_dummy;
    arc_t *arc;
    flow_t flow;
    

    node = net->nodes;
    stop = (void *)net->stop_nodes;

    for( node++; node < (node_t *)stop; node++ )
    {
        arc = node->basic_arc;
        flow = node->flow;
        if( arc >= dummy && arc < stop_dummy )
        {
            if( ABS(flow) > (flow_t)net->feas_tol )
            {
                printf( "PRIMAL NETWORK SIMPLEX: " );
                printf( "artificial arc with nonzero flow, node %d (%" PRId64 ")\n",
                        node->number, flow );
            }
        }
        else
        {
            if( flow < (flow_t)(-net->feas_tol)
               || flow - (flow_t)1 > (flow_t)net->feas_tol )
            {
                printf( "PRIMAL NETWORK SIMPLEX: " );
                printf( "basis primal infeasible (%" PRId64 ")\n", flow );
                net->feasible = 0;
                return 1;
            }
        }
    }
    
    net->feasible = 1;
    
    return 0;
}










#ifdef _PROTO_
LONG dual_feasible( network_t *net )
#else
LONG dual_feasible(  net )
    network_t *net;
#endif
{
    arc_t         *arc;
    arc_t         *stop     = net->stop_arcs;
    cost_t        red_cost;
    
    LONG i = 0;
    

    for( i= 0, arc = net->arcs; arc < stop; arc++, i++ )
    {
        red_cost = arc->cost - arc->tail->potential 
            + arc->head->potential;
        switch( arc->ident )
        {
        case BASIC:
#ifdef AT_ZERO
        case AT_ZERO:
            if( ABS(red_cost) > (cost_t)net->feas_tol )
#ifdef DEBUG
                printf("%d %d %d %ld\n", arc->tail->number, arc->head->number,
                       arc->ident, red_cost );
#else
                goto DUAL_INFEAS;
#endif
            
            break;
#endif
        case AT_LOWER:
            if( red_cost < (cost_t)-net->feas_tol )
#ifdef DEBUG
                printf("%d %d %d %ld\n", arc->tail->number, arc->head->number,
                       arc->ident, red_cost );
#else
                goto DUAL_INFEAS;
#endif

            break;
        case AT_UPPER:
            if( red_cost > (cost_t)net->feas_tol )
#ifdef DEBUG
                printf("%d %d %d %ld\n", arc->tail->number, arc->head->number,
                       arc->ident, red_cost );
#else
                goto DUAL_INFEAS;
#endif

            break;
        case FIXED:
        default:
            break;
        }
    }
    
    return 0;
    
DUAL_INFEAS:
    printf("DUAL NETWORK SIMPLEX: " );
    printf("basis dual infeasible\n" );
    return 1;
}







#ifdef _PROTO_
LONG getfree( 
            network_t *net
            )
#else
LONG getfree( net )
     network_t *net;
#endif
{  
    FREE( net->nodes );
    FREE( net->arcs );
    FREE( net->dummy_arcs );
    FREE( net->sorted_arcs);
    net->nodes = net->stop_nodes = NULL;
    net->arcs = net->stop_arcs = NULL;
    net->dummy_arcs = net->stop_dummy = NULL;
    net->sorted_arcs = NULL;

    return 0;
}



