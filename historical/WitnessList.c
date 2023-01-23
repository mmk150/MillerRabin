/* Here is the code used to find and save the non-witnesses */

#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <time.h>
#include <string.h>

/* Fast modular arithmetic exponentiation  algorithm */
unsigned long long powermod(unsigned long long a,unsigned long long exp,unsigned long long modulus)
{unsigned long long aleph,a2;
   aleph=a;  a2=0;
  if(exp==1){
    return a;
  }
  if(exp % 2 ==0) {
    aleph=aleph*aleph % modulus;
    return (powermod(aleph,exp/2,modulus));
  }
  else{ 
    a2 = aleph * aleph;
    a2 = a2 % modulus;
    return(aleph*powermod(a2,(exp-1)/2,modulus) % modulus);
  }
 
 
}


int main(int argc, char *argv[]){
	unsigned long long i,j,l,wcount,p,temp,twocount,aleph,ver,start,end,r,t;
	
	char buff[50]="WitnessList";
	FILE *fd=NULL;
	char fileID[50]="WitnessList";
	char nums[10];
	
	start = 3;
	end=1000000;
      j=0, l = 0;
    /* Outside loop */
	  for (i = start; i < end; i = i + 2){
		  /*Prepare some file i/o: */
		  strcpy(fileID, buff);
		  itoa(i, nums, 10);
		  printf("%s", nums);
		  strcat(fileID, nums);
		  strcat(fileID, ".txt");
		  printf("%s", fileID);
		  fd = fopen(fileID, "w+");
		  /* File stuff done*/
		  p = i - 1;
		  temp = p;
		  twocount = 0;
		  /* Extract all powers of two */
		  while (temp % 2 == 0){
			  temp = temp / 2;
			  twocount = twocount + 1;
		  }
		  wcount = 0;
		  /* inside loop to check congruence conditions */

		  for (j = 2; j < i; j = j + 1){
			  ver = 0;


			  aleph = powermod(j, temp, i);
			  if (aleph == 1){
				  ver = 1;
			  }

			  if (ver == 0){
				  for (l = 0; l < twocount; l++){
					  if (aleph == i - 1){
						  ver = 1;
						  break;
					  }
					  aleph = powermod(aleph, 2, i);
				  }
			  }
			  if (ver == 0){

				  wcount++;
				  continue;
			  }

			  if (ver == 1){
				  fprintf(fd, "%d\n", j);
			  }
		  }
		  fclose(fd);
	  }
    

    return 0;
}
	
	
	
