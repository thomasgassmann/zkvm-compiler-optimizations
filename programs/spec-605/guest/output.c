/**************************************************************************
OUTPUT.C of ZIB optimizer MCF, SPEC version

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
/*  LAST EDIT: Wed May 26 00:05:32 2010 by Loebel (opt0.zib.de)  */
/*  $Id: output.c,v 1.12 2010/05/26 08:26:29 bzfloebe Exp $  */



#include "output.h"





#ifdef _PROTO_
LONG write_circulations(
                   char *outfile,
                   network_t *net
                   )
#else
LONG write_circulations( outfile, net )
     char *outfile;
     network_t *net;
#endif 
{
  // FILE *out = NULL;
  arc_t *block;
  arc_t *arc;
  arc_t *arc2;
  arc_t *first_impl = net->stop_arcs - net->m_impl;
  
  // if(( out = fopen( outfile, "w" )) == NULL )
  //   return -1;
  
  refresh_neighbour_lists( net, &getArcPosition );
  
  for( block = net->nodes[net->n].firstout; block; block = block->nextout )
  {
    if( block->flow )
    {
      printf( "()\n" );
      
      arc = block;
      while( arc )
      {
        if( arc >= first_impl )
          printf( "***\n" );
        
        printf( "%d\n", - arc->head->number );
        arc2 = arc->head[net->n_trips].firstout; 
        for( ; arc2; arc2 = arc2->nextout )
          if( arc2->flow )
            break;
        if( !arc2 )
        {
          // fclose( out );
          return -1;
        }
        
        if( arc2->head->number )
          arc = arc2;
        else
          arc = NULL;
      }
    }
  }
  
  
  // fclose(out);
  
  return 0;
}





#ifdef _PROTO_
LONG write_objective_value(
                   char *outfile,
                   network_t *net
                   )
#else
LONG write_objective_value( outfile, net )
     char *outfile;
     network_t *net;
#endif 
{
  // FILE *out = NULL;
  
  // if(( out = fopen( outfile, "w" )) == NULL )
  //   return -1;
  
  printf( "%.0f\n", flow_cost(net) );
  
  // fclose(out);
  
  return 0;
}
