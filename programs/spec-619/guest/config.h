/* $Id: config.h,v 1.7 2004/05/03 19:45:58 larsonre Exp $ */

/*############################################################################*/

#ifndef _CONFIG_H_
#define _CONFIG_H_

/*############################################################################*/

#if !defined(LARGE_WORKLOAD)
#define SIZE   (100)
#define SIZE_X (1*SIZE)
#define SIZE_Y (1*SIZE)
#define SIZE_Z (130)
#else
#define SIZE   (100)
#define SIZE_X (2*SIZE)
#define SIZE_Y (2*SIZE)
#define SIZE_Z (2*130)
#endif /* !LARGE_WORKLOAD */

#define OMEGA (1.95)

#define OUTPUT_PRECISION float

#define BOOL int
#define TRUE (-1)
#define FALSE (0)

/*############################################################################*/

#endif /* _CONFIG_H_ */
