/**************************************************************************
MCF.H of ZIB optimizer MCF, SPEC version

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
/*  LAST EDIT: Wed May 26 00:06:03 2010 by Loebel (opt0.zib.de)  */
/*  $Id: mcf.c,v 1.17 2010/05/26 08:26:29 bzfloebe Exp $  */



#include "mcf.h"
// #include "time.h"

#define REPORT

extern LONG min_impl_duration;
network_t net;





#ifdef _PROTO_
LONG global_opt( void )
#else
LONG global_opt( )
#endif
{
  LONG new_arcs;
  LONG residual_nb_it = 1;
  
  
  new_arcs = -1;

  while( new_arcs )
  {
#ifdef REPORT
    printf( "active arcs                : %d\n", net.m );
#endif
    
    primal_net_simplex( &net );
    
#ifdef REPORT
    printf( "simplex iterations         : %d\n", net.iterations );
    printf( "objective value            : %0.0f\n", flow_cost(&net) );
#endif
    
    
#if defined AT_HOME
    printf( "iterations                 : %ld\n", residual_nb_it );
#endif
    
    
    if( net.m_impl )
    {
      new_arcs = suspend_impl( &net, (cost_t)-1, 0 );
      
      if ( new_arcs < 0)
      {
#ifdef REPORT
        printf( "not enough memory, exit(-1)\n" );
#endif
        
        exit(-1);
      }
#ifdef REPORT
      if( new_arcs )
        printf( "erased arcs                : %d\n", new_arcs );
#endif
    }
    else 
    {
      refreshPositions(&net, &getOriginalArcPosition,net.m);
    }
    
    
    new_arcs = price_out_impl( &net );
    
#ifdef REPORT
    if( new_arcs )
      printf( "new implicit arcs          : %" PRId64 "\n", new_arcs );
#endif
    
    if( new_arcs < 0 )
    {
#ifdef REPORT
      printf( "not enough memory, exit(-1)\n" );
#endif
      
      exit(-1);
    }
    
#ifndef REPORT
    printf( "\n" );
#endif
    
    residual_nb_it++;
  }
  
  net.optcost = flow_cost(&net);
  printf( "checksum                   : %0.0f\n", net.optcost );
  
  return 0;
}






void cmain()
{
  // int outnum; 
  // char outfile[80];
  // if( argc < 2 )
  //   return -1;
  char *input_file = "";
  
#ifndef SPEC
  time_t startTime, endTime;
  time(&startTime);
#endif
  
  printf( "\nMCF SPEC CPU version 1.11\n" );
  printf( "Copyright (c) 1998-2000 Zuse Institut Berlin (ZIB)\n" );
  printf( "Copyright (c) 2000-2002 Andreas Loebel & ZIB\n" );
  printf( "Copyright (c) 2003-2005 Andreas Loebel\n" );
  printf( "Copyright (c) 2006-2010 Dres. Loebel, Borndoerfer & Weider "\
          "GbR (LBW)\n" );
  printf( "\n" );
  
  //omp_set_num_threads(1);
  
  memset( (void *)(&net), 0, (size_t)sizeof(network_t) );
  net.bigM = (LONG)BIGM;
  
  strcpy( net.inputfile, input_file );
  // if (argc == 3) {
  //    outnum = atoi(argv[2]);
  //    sprintf(outfile,"mcf.%d.out",outnum);
  // } else {
  //    strcpy(outfile,"mcf.out"); 
  // }  
 
  if( read_min( &net ) )
  {
    printf( "read error, exit\n" );
    getfree( &net );
    return;
  }
  
#ifndef SPEC
#ifdef _OPENMP
  printf( "number of threads          : %d\n", omp_get_max_threads() );
#else
  printf( "single threaded\n" );
#endif
#endif
  
#if defined(REPORT) || defined(SPEC)
  printf( "nodes                      : %d\n", net.n_trips );
#endif
  
  
  primal_start_artificial( &net );
  global_opt( );
  
  
  
  if( write_objective_value( "no-out-file", &net ) )
  {
    getfree( &net );
    return;    
  }
  
  
  getfree( &net );
#ifndef SPEC
  time(&endTime);
  printf("runtime = %ld seconds\n",endTime - startTime);
#endif
  
  
#ifdef REPORT
  printf( "done\n" );
#endif
  
  
  return;
}
