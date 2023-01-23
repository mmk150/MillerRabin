
%This function when called will go through a nonwitness text file for n=alpha. 

function [W]=CollectData(alpha);
format long;
last=1;

intnum=10 ;
sp=1/intnum;
    
str= 'WitnessList';
str= strcat(str, int2str(alpha),'.txt');
fileID=fopen(str);
C=textscan(fileID,'%d');
fclose(fileID);
C=C{1};
N=length(C);
bincount=zeros(1);
final=ceil(1/sp);

    for count=1:final
        bincount(count)=0;
    end

    for j=1:N
       num=double(C(j));
       num=num/alpha;
        for k=sp:sp:1
        if( k-sp <=num && num<=k)
            index=round(k*intnum);
            bincount(index)=bincount(index)+1;
        end
        end
    end

    Z{last}=bincount;
    last=last+1;
    W=Z;
