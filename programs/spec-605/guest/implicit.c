/**************************************************************************
IMPLICIT.C of ZIB optimizer MCF, SPEC version

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
/*  LAST EDIT: Tue May 25 23:46:30 2010 by Loebel (opt0.zib.de)  */
/*  $Id: implicit.c,v 1.21 2010/05/25 21:58:44 bzfloebe Exp $  */


#if defined(SPEC)
# include "spec_qsort.h"
#endif

#include "implicit.h"
#include <zkvm.h>

#ifdef _PROTO_
static int arc_compare( arc_t **a1, arc_t **a2 )
#else
static int arc_compare( a1, a2 )
arc_t **a1;
arc_t **a2;
#endif
{
  if( (*a1)->flow > (*a2)->flow )
    return 1;
  if( (*a1)->flow < (*a2)->flow )
    return -1;
  if( (*a1)->id < (*a2)->id )
    return -1;

    return 1;
}

#ifdef _PROTO_
LONG refreshArcPointers(network_t *net, LONG (*getPos)(network_t *, LONG), arc_t* sorted_array)
#else
refreshArcPointers(net, getPos, sorted_array)
network_t *net;
arc_t* sorted_array;
LONG (*getPos)(network_t *, LONG);
#endif
{
  node_p node;
  LONG i;

#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp parallel for private(node)
#endif
    for (i = 0; i <= net->n; i++) {
      node = net->nodes + i;
      if (node->basic_arc && node->basic_arc->id >= 0)
          node->basic_arc = &sorted_array[getPos(net, node->basic_arc->id)];
      if (node->firstin && node->firstin->id >= 0)
          node->firstin = &sorted_array[getPos(net, node->firstin->id)];
      if (node->firstout && node->firstout->id >= 0)
          node->firstout = &sorted_array[getPos(net, node->firstout->id)];
    }

    return 0;
}


#ifdef _PROTO_
LONG refreshPositions( network_t *net, LONG (*getPos)(network_t *, LONG),LONG new_m)
#else
LONG refreshPositions( net, getPos, new_m)
network_t *net;
LONG (*getPos)(LONG);
LONG new_m;
#endif
{
  arc_t *arc, *sorted_array;
    LONG position, new_position;

    sorted_array = net->sorted_arcs;
    refreshArcPointers(net, getPos, sorted_array);

#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp parallel for private(arc, new_position)
#endif
  for (position = 0; position < new_m; position++) {
    arc = net->arcs + position;
    if(arc->id < 0)
      continue;
    new_position = getPos(net, arc->id);
    sorted_array[new_position] = *arc;
  }

  arc = net->arcs;
  net->arcs = sorted_array;
  net->sorted_arcs = arc;
  net->stop_arcs = sorted_array + new_m;

  return 0;
}

#ifdef _PROTO_
void marc_arcs(network_t* net, LONG *new_arcs, LONG *new_arcs_array, arc_p **arcs_pointer_sorted)
#else
void marc_arcs(net, new_arcs, new_arcs_array, arcs_pointer_sorted)
network_t* net;
LONG *new_arcs;
LONG *new_arcs_array;
arc_p **arcs_pointer_sorted;
#endif
{

  LONG max_new_arcs;
  arc_p **positions;
  LONG *values;
  LONG global_new = 0;
  LONG best_pos = 0;
  LONG start_id, i;
  arc_t* arc;
  
  #if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
  LONG num_threads = omp_get_max_threads();
  #else
  LONG num_threads = 1;
  #endif

  positions = (arc_p**) malloc(num_threads * sizeof(arc_p*));
  values = (LONG*) malloc(num_threads * sizeof(LONG));

  if (net->n_trips <= MAX_NB_TRIPS_FOR_SMALL_NET)
    max_new_arcs = net->max_residual_new_m - MAX_NEW_ARCS_PUFFER_SMALL_NET;
  else
    max_new_arcs = net->max_residual_new_m - MAX_NEW_ARCS_PUFFER_LARGE_NET;

  *new_arcs = 0;


  for(i = 0; i< num_threads; i++) {
    *new_arcs += new_arcs_array[i];
    positions[i] = arcs_pointer_sorted[i];
    values[i] = 0;
  }


  start_id = net->m;
  while(global_new < *new_arcs && global_new < max_new_arcs) {
    if (values[0] < new_arcs_array[0]) {
            arc = *positions[0];
            best_pos = 0;
    }
    else {
      arc = 0;
    }
        for (i = 1; i < num_threads; i++) {
            if ((values[i] < new_arcs_array[i]) && ((!arc) || arc_compare(positions[i], &arc) < 0)) {
                arc = *positions[i];
                best_pos = i;
            }
        }
//        if (global_new >= 3999000)
//            printf("global_new %ld redcost %ld arcid %d  \n",global_new,arc->flow, arc->id);
        arc->id = start_id++;
        arc->flow = 1;
        global_new++;
        positions[best_pos]++;
        values[best_pos]++;


  }
//    arc_t* nu;
//    nu->id = 0;



  *new_arcs = 0;
  for (i = 0; i< num_threads; i++) {
    *new_arcs += values[i];
  }
  net->max_elems = K;
  net->nr_group = ( (*new_arcs + net->m -1) / K ) + 1;
  if ((*new_arcs + net->m) % K != 0)
      net->full_groups = net->nr_group - (K - ((*new_arcs + net->m) % K));
  else
      net->full_groups = net->nr_group;
  while (net->full_groups < 0) {
    net->full_groups = net->nr_group + net->full_groups;
    net->max_elems--;
  }

    free(positions);
    free(values);

}
#ifdef _PROTO_
LONG resize_prob( network_t *net )
#else
LONG resize_prob( net )
     network_t *net;
#endif
{
    arc_t *arc, *old_arcs;
    node_t *node, *stop, *root;
    size_t off;
            
    
    assert( net->max_new_m >= 3 );


    net->max_m += net->max_new_m;
    net->max_residual_new_m += net->max_new_m;

#if defined AT_HOME
    printf( "\nresize arcs to %4ld MB (%ld elements a %lu byte)\n\n",
            net->max_m * sizeof(arc_t) / 0x100000,
            net->max_m,
            (unsigned LONG)sizeof(arc_t) );
    // fflush( stdout );
#endif


    arc = (arc_t *) realloc( net->arcs, net->max_m * sizeof(arc_t) );
    if( !arc )
    {
        printf( "network %s: not enough memory\n", net->inputfile );
        // fflush( stdout );
        return -1;
    }
    
    old_arcs = net->arcs;

    net->arcs = arc;
    net->stop_arcs = arc + net->m;

    root = node = net->nodes;
    for(node++, stop = net->stop_nodes; node < stop; node++ ) {
       if( node->basic_arc && node->pred != root) {
          off = node->basic_arc - old_arcs;
            node->basic_arc = (arc_t *)(net->arcs + off);
        }
    }

    arc = (arc_t *) realloc( net->sorted_arcs, net->max_m * sizeof(arc_t) );
    net->sorted_arcs = arc;
        
    return 0;
}







#ifdef _PROTO_
void insert_new_arc(arc_t *newarc, LONG newpos, node_t *tail, node_t *head,
    cost_t cost, cost_t red_cost, LONG m, LONG number)
#else
void insert_new_arc( newarc, newpos, tail, head, cost, red_cost, m, number)
     arc_t *newarc;
     LONG newpos;
     node_t *tail;
     node_t *head;
     cost_t cost;
     cost_t red_cost;
     LONG m;
     LONG number;
#endif
{
    LONG pos;

    newarc[newpos].tail      = tail;
    newarc[newpos].head      = head;
    newarc[newpos].org_cost  = cost;
    newarc[newpos].cost      = cost;
    newarc[newpos].flow      = (flow_t)red_cost;
    newarc[newpos].id        = number;

    pos = newpos+1;
    while( pos-1 && red_cost > (cost_t)newarc[pos/2-1].flow )
    {
        newarc[pos-1].tail     = newarc[pos/2-1].tail;
        newarc[pos-1].head     = newarc[pos/2-1].head;
        newarc[pos-1].cost     = newarc[pos/2-1].cost;
        newarc[pos-1].org_cost = newarc[pos/2-1].cost;
        newarc[pos-1].flow     = newarc[pos/2-1].flow;
        newarc[pos-1].id       = newarc[pos/2-1].id;

        pos = pos/2;
        newarc[pos-1].tail     = tail;
        newarc[pos-1].head     = head;
        newarc[pos-1].cost     = cost;
        newarc[pos-1].org_cost = cost;
        newarc[pos-1].flow     = (flow_t)red_cost;
        newarc[pos-1].id       = number;
    }
    
    return;
}   






#ifdef _PROTO_
void replace_weaker_arc( arc_t *newarc, node_t *tail, node_t *head,
                         cost_t cost, cost_t red_cost,LONG max_new_par_residual_new_arcs, LONG number)
#else
void replace_weaker_arc( net, newarc, tail, head, cost, red_cost, max_new_par_residual_new_arcs, number)
     network *net;
     arc_t *newarc;
     node_t *tail;
     node_t *head;
     cost_t cost;
     cost_t red_cost;
     LONG max_new_par_residual_new_arcs;
     LONG number;
#endif
{
    LONG pos;
    LONG cmp;

    newarc[0].tail     = tail;
    newarc[0].head     = head;
    newarc[0].org_cost = cost;
    newarc[0].cost     = cost;
    newarc[0].flow     = (flow_t)red_cost; 
    newarc[0].id       = number;
                    
    pos = 1;
    cmp = (newarc[1].flow > newarc[2].flow) ? 2 : 3;
    while( cmp <= max_new_par_residual_new_arcs && red_cost < newarc[cmp-1].flow )
    {
        newarc[pos-1].tail = newarc[cmp-1].tail;
        newarc[pos-1].head = newarc[cmp-1].head;
        newarc[pos-1].cost = newarc[cmp-1].cost;
        newarc[pos-1].org_cost = newarc[cmp-1].cost;
        newarc[pos-1].flow = newarc[cmp-1].flow;
        newarc[pos-1].id   = newarc[cmp-1].id;
        
        newarc[cmp-1].tail = tail;
        newarc[cmp-1].head = head;
        newarc[cmp-1].cost = cost;
        newarc[cmp-1].org_cost = cost;
        newarc[cmp-1].flow = (flow_t)red_cost; 
        newarc[cmp-1].id   = number;
        pos = cmp;
        cmp *= 2;
        if( cmp + 1 <= max_new_par_residual_new_arcs )
            if( newarc[cmp-1].flow < newarc[cmp].flow )
                cmp++;
    }

    return;
}   




#if defined AT_HOME
#include <sys/time.h>
double Get_Time( void  ) 
{
    struct timeval tp;
    struct timezone tzp;
    if( gettimeofday( &tp, &tzp ) == 0 )
        return (double)(tp.tv_sec) + (double)(tp.tv_usec)/1.0e6;
    else
        return 0.0;
}
static double wall_time = 0; 
#endif

#ifdef _PROTO_
void calculate_max_redcost(network_t *net, LONG* max_redcost, arc_t*** arcs_pointer_sorted, LONG num_threads)
#else
void calculate_max_redcost(max_redcost, arcs_pointer_sorted, num_threads)
LONG* max_redcost;
arc_t*** arcs_pointer_sorted;
LONG num_threads;
#endif
{
  LONG i;
  *max_redcost = 0;
  for (i = 0; i < num_threads; i++)
  {
    if (arcs_pointer_sorted[i][0]->flow > *max_redcost)
      *max_redcost = arcs_pointer_sorted[i][0]->flow;
  }
}

#ifdef _PROTO_
LONG switch_arcs(network_t* net, LONG *num_del_arcs, arc_t** deleted_arcs, arc_t* arcnew, int thread, LONG max_new_par_residual_new_arcs, LONG size_del, LONG num_threads)
#else
LONG switch_arcs(net, num_del_arcs, deleted_arcs, arcnew, thread, max_new_par_residual_new_arcs, size_del, num_threads)
network_t* net;
LONG *num_del_arcs;
arc_t** deleted_arcs;
arc_t* arcnew;
int thread;
LONG max_new_par_residual_new_arcs;
LONG size_del;
LONG num_threads;
#endif
{
    LONG i, j, h, number_of_arcs, count=0;
    arc_t *test_arc, copy;

   for (i = 0, j = thread; i < num_threads; i++, j = (j + 1) % num_threads)
   {
#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp barrier
#endif
     number_of_arcs = (num_del_arcs[j] < size_del) ? num_del_arcs[j] : size_del;
     for (h = 0; h < number_of_arcs; h++)
     {
       test_arc = &deleted_arcs[j][h];
//       if (test_arc->flow == 0)
//         continue;
       if (!test_arc->ident && ((test_arc->flow < arcnew[0].flow) || (test_arc->flow == arcnew[0].flow &&
           test_arc->id < arcnew[0].id)))
       {
         copy = *test_arc;
         count++;
         *test_arc = arcnew[0];
         replace_weaker_arc( arcnew, copy.tail, copy.head, copy.cost, copy.flow, max_new_par_residual_new_arcs, copy.id );
       }
     }

   }

   return count;
}

#ifdef _PROTO_
LONG price_out_impl( network_t *net )
#else
LONG price_out_impl( net )
     network_t *net;
#endif
{
    LONG i;
    LONG trips;
    LONG new_arcs = 0;
    LONG resized = 0;
    LONG latest;
    LONG min_impl_duration = 15;
    LONG max_new_par_residual_new_arcs;
    int thread;
    LONG *new_arcs_array;
    LONG id, list_size, *num_del_arcs;
    arc_p **arcs_pointer_sorted, *deleted_arcs;
    LONG max_redcost;
    short first_replace = 1, local_first_replace;
    LONG count = 0;
    LONG num_switch_iterations;
  LONG size_del;

    register list_elem *first_list_elem;
    register list_elem *new_list_elem;
    register list_elem* iterator;

    register cost_t bigM = net->bigM;
    register cost_t head_potential;
    register cost_t arc_cost = 30;
    register cost_t red_cost;
    register cost_t bigM_minus_min_impl_duration;
        
    register arc_t *arcout, *arcin, *arcnew, *stop, *sorted_array, *arc;
    register node_t *tail, *head;

#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
    LONG num_threads = omp_get_max_threads();
#else
    LONG num_threads = 1;
#endif


    new_arcs_array = (LONG*) malloc(num_threads * sizeof(LONG));
    num_del_arcs = (LONG*) malloc(num_threads * sizeof(LONG));
    arcs_pointer_sorted = (arc_p**) malloc(num_threads * sizeof(arc_p*));
    deleted_arcs = (arc_p*) malloc(num_threads * sizeof(arc_p));

#if defined AT_HOME
    wall_time -= Get_Time();
#endif

    
    bigM_minus_min_impl_duration = (cost_t)bigM - min_impl_duration;
    

    
    if( net->n_trips <= MAX_NB_TRIPS_FOR_SMALL_NET )
    {
      if( net->m + net->max_new_m > net->max_m 
          &&
          (net->n_trips*net->n_trips)/2 + net->m > net->max_m
          )
      {
        resized = 1;
        if( resize_prob( net ) )
          return -1;
        
        refresh_neighbour_lists( net, &getOriginalArcPosition );
      }
    }
    else
    {
      if( net->m + net->max_new_m > net->max_m 
          &&
          (net->n_trips*net->n_trips)/2 + net->m > net->max_m
          )
      {
        resized = 1;
        if( resize_prob( net ) )
          return -1;
        
        refresh_neighbour_lists( net, &getOriginalArcPosition );
      }
    }

    if (net->n_trips <= MAX_NB_TRIPS_FOR_SMALL_NET)
        num_switch_iterations = ITERATIONS_FOR_SMALL_NET;
    else
        num_switch_iterations = ITERATIONS_FOR_BIG_NET;

    sorted_array = net->sorted_arcs;
    if (!sorted_array)
      return -1;
    max_new_par_residual_new_arcs = net->max_residual_new_m / num_threads;
  first_replace = 1;
    size_del = net->max_m/num_threads;
#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp parallel private(local_first_replace,count,arc,max_redcost,list_size,id, thread, stop, red_cost, arcin, head_potential, iterator, head, tail, latest, new_list_elem, first_list_elem, arcout, i, arcnew, trips )
#endif
    {
      //printf("del %d\n", size_del);
      local_first_replace = 1;
      max_redcost = 0;
      count =0;

#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
  thread = omp_get_thread_num();
#else
  thread = 0;
#endif

      deleted_arcs[thread] = &sorted_array[size_del * thread];
      num_del_arcs[thread] = 0;
        new_arcs_array[thread] = 0;
      arcnew = net->stop_arcs + thread * max_new_par_residual_new_arcs;
      trips = net->n_trips;
      id = 0;
      list_size = -1;
      arcs_pointer_sorted[thread] = (arc_p*) calloc (max_new_par_residual_new_arcs, sizeof(arc_p));


      for (i = 0; i < max_new_par_residual_new_arcs; i++) {
        arcs_pointer_sorted[thread][i] = &arcnew[i];
      }

      arcout = net->arcs;
      for( i = 0; i < trips && arcout[1].ident == FIXED; i++, arcout += 3);

      first_list_elem = (list_elem *)NULL;
      for( ; i < trips; i++, arcout += 3 )
      {
        if (!first_replace) {
            calculate_max_redcost(net, &max_redcost, arcs_pointer_sorted, num_threads);
        }

        if ( i % num_switch_iterations == 0) {
#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp barrier
#endif
            calculate_max_redcost(net, &max_redcost, arcs_pointer_sorted, num_threads);
            if (!first_replace) {
              //printf("thread %d count %ld del_size %ld\n", thread, count, size_del);
              num_del_arcs[thread] = count;
              switch_arcs(net, num_del_arcs, deleted_arcs, arcnew, thread, max_new_par_residual_new_arcs, size_del, num_threads);
              count = 0;
              num_del_arcs[thread] = 0;
          }
        }

        if( arcout[1].ident != FIXED )
        {
          new_list_elem = (list_elem*) calloc(1, sizeof(list_elem));
          new_list_elem->next = first_list_elem;
          new_list_elem->arc = arcout + 1;
          first_list_elem = new_list_elem;
          list_size++;
        }

        if( arcout->ident == FIXED || i % num_threads != thread)
        {
          id += list_size;
          continue;
        }

        head = arcout->head;
        latest = head->time - arcout->org_cost
            + (LONG)bigM_minus_min_impl_duration;

        head_potential = head->potential;

        iterator = first_list_elem->next;
        while( iterator )
        {

          arcin = iterator->arc;
          tail = arcin->tail;

          if( tail->time + arcin->org_cost > latest )
          {
            iterator = iterator->next;
            id++;
            continue;
          }

          red_cost = arc_cost - tail->potential + head->potential;

          if( red_cost < 0 )
          {

            if( new_arcs_array[thread] < max_new_par_residual_new_arcs)
            {
              insert_new_arc( arcnew, new_arcs_array[thread], tail, head,
                  arc_cost, red_cost, net->m, id);
              new_arcs_array[thread]++;
            }
            else if( (cost_t)arcnew[0].flow > red_cost ) {
              if (local_first_replace) {
                first_replace = 0;
                local_first_replace = 0;
              }

              deleted_arcs[thread][num_del_arcs[thread]] = arcnew[0];
              num_del_arcs[thread]++;
              count++;
              replace_weaker_arc( arcnew, tail, head, arc_cost, red_cost, max_new_par_residual_new_arcs, id);
            }
            else if (red_cost < max_redcost ) {
              arc = &deleted_arcs[thread][num_del_arcs[thread]++];
              arc->tail     = tail;
              arc->head     = head;
              arc->org_cost = arc_cost;
              arc->cost     = arc_cost;
              arc->flow     = (flow_t)red_cost;
              arc->id       = id;
              count++;
            }
            if (num_del_arcs[thread] == size_del)
            {
              num_del_arcs[thread] = 0;
            }
          }

          iterator = iterator->next;
          id++;
        }

      }

      num_del_arcs[thread] = count;
      while (!first_replace) {
#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp barrier
#endif
        first_replace = 1;
        //printf("Schleife vorher thread %d count %ld del_size %ld\n", thread, count, size_del);
          count = switch_arcs(net, num_del_arcs, deleted_arcs, arcnew, thread, max_new_par_residual_new_arcs, size_del, num_threads);
          //printf("Schleife nachher thread %d count %ld del_size %ld\n", thread, count, size_del);
          if (count)
            first_replace = 0;
#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp barrier
#endif
      }

      while (first_list_elem->next) {
        new_list_elem = first_list_elem;
        first_list_elem = first_list_elem->next;
        free(new_list_elem);
      }
      free(first_list_elem);

#if defined(SPEC)
        spec_qsort(arcs_pointer_sorted[thread], new_arcs_array[thread], sizeof(arc_p),
                (int (*)(const void *, const void *))arc_compare);
#else
        qsort(arcs_pointer_sorted[thread], new_arcs_array[thread], sizeof(arc_p),
                (int (*)(const void *, const void *))arc_compare);
#endif

#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp barrier
#pragma omp master
#endif
      {
          marc_arcs(net, &new_arcs, new_arcs_array, arcs_pointer_sorted);
      }

#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp barrier
#endif
  free(arcs_pointer_sorted[thread]);
      if( new_arcs_array[thread] )
      {
        arcnew = net->stop_arcs + thread * max_new_par_residual_new_arcs;
        stop = arcnew + new_arcs_array[thread];
        if( resized )
        {
          for( ; arcnew != stop; arcnew++ )
          {
            if (arcnew->flow == 1) {
                arcnew->flow = (flow_t)0;
                arcnew->ident = AT_LOWER;
#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp critical
#endif
	        sorted_array[getArcPosition(net, arcnew->id)] = *arcnew;
            }
          }
        }
        else
        {
          for( ; arcnew != stop; arcnew++ )
          {
            if (arcnew->flow == 1) {
              arcnew->flow = (flow_t)0;
              arcnew->ident = AT_LOWER;
              arcnew->nextout = arcnew->tail->firstout;
              arcnew->tail->firstout = arcnew;
              arcnew->nextin = arcnew->head->firstin;
              arcnew->head->firstin = arcnew;
#if (defined(_OPENMP) || defined(SPEC_OPENMP)) && !defined(SPEC_SUPPRESS_OPENMP) && !defined(SPEC_AUTO_SUPPRESS_OPENMP)
#pragma omp critical
#endif
              sorted_array[getArcPosition(net, arcnew->id)] = *arcnew;
            }
          }
        }
      }
    }

        net->m_impl += new_arcs;
        net->max_residual_new_m -= new_arcs;
        refreshPositions(net, &getArcPosition, net->m);
        net->m = net->m + new_arcs;
        net->stop_arcs = net->arcs + net->m;

#ifdef DEBUG
       arc_t* arc = net->arcs;
       for (i=0;arc < net->stop_arcs; arc++, i++)
         if (!arc->head) {
           printf("arc %ld is null\n", i);
         }
#endif
    

#if defined AT_HOME
    wall_time += Get_Time();
    printf( "total time price_out_impl(): %0.0f\n", wall_time );
#endif

    free(new_arcs_array);
    free(num_del_arcs);
    free(arcs_pointer_sorted);
    free(deleted_arcs);

    return new_arcs;
}   




#ifdef _PROTO_
LONG suspend_impl( network_t *net, cost_t threshold, LONG all )
#else
LONG suspend_impl( net, threshold, all )
     network_t *net;
     cost_t threshold;
     LONG all;
#endif
{
    LONG susp;
    
    cost_t red_cost;
    arc_t *arc;
    LONG stop, startid;

    net->max_elems = K;
    net->nr_group = ( (net->m -1) / K ) + 1;
    net->full_groups = net->nr_group - (K - (net->m % K));
  while (net->full_groups < 0) {
    net->full_groups = net->nr_group + net->full_groups;
    net->max_elems--;
  }

    if( all ) {
        susp = net->m_impl;
    }
    else
    {

        startid = net->m - net->m_impl;
        for( stop = net->m - net->m_impl, susp = 0; stop < net->m;  stop++)
        {
          arc = net->arcs + getArcPosition(net, stop);
            if( arc->ident == AT_LOWER )
                red_cost = arc->cost - arc->tail->potential
                        + arc->head->potential;
            else
            {
                red_cost = (cost_t)-2;

                if( arc->ident == BASIC )
                {
                    if( !(arc->tail->basic_arc == arc) )
                      arc->head->basic_arc = arc;
                }
            }

            if( red_cost > threshold ) {
                susp++;
                arc->id = DELETED;
            }
            else
            {
              arc->id = startid;
                startid++;
            }
        }
    }
    
        
#if defined AT_HOME
    printf( "\nremove %ld arcs\n\n", susp );
    fflush( stdout );
#endif

    if( susp )
    {
        net->m_impl -= susp;
        net->max_residual_new_m += susp;
        
        net->max_elems = K;
        net->nr_group = ( (net->m - susp -1) / K ) + 1;
        if ((net->m - susp) % K != 0)
           net->full_groups = net->nr_group - (K - ((net->m - susp) % K));
        else
           net->full_groups = net->nr_group;
      while (net->full_groups < 0) {
        net->full_groups = net->nr_group + net->full_groups;
        net->max_elems--;
      }
        refreshPositions(net, &getOriginalArcPosition, net->m);
      net->m -= susp;
        net->stop_arcs -= susp;
        refresh_neighbour_lists( net, &getOriginalArcPosition );
    }

    return susp;
}


