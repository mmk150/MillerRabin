import os
import numpy as np
import matplotlib.pyplot as plt
import pandas as pd


directory="witnesses"
mainpath=os.getcwd()

def clean_item(item):
    copy=item.strip()
    copy=copy.strip(",")
    return copy

for filename in os.listdir(directory):
    number=filename.strip().strip(".txt")
    
    full_filename=os.path.join(directory,filename)
    df=pd.read_csv(full_filename, sep=' ',header=None, names=['Nonwitnesses'])
    df=df.applymap(clean_item)
    if df.empty or df.loc[0]['Nonwitnesses']=='prime':
        continue
    else:
        image=str(number) + ".png"
        number=int(number)
        arr=[x for x in range(2,number)]
        df['Nonwitnesses']=pd.to_numeric(df['Nonwitnesses'])
        nwcount=0
        for r in df.iterrows():
            x=r[1]['Nonwitnesses']
            arr[x]=-1
            nwcount+=1
        
        np_arr=np.array(arr)
        fig, ax = plt.subplots()
        Nonwitnesses=nwcount
        Witnesses=number-Nonwitnesses-3
        # There is no need to count 1,N-1 or N 
        textstr = '\n'.join((
            r'$Witnesses=%d$' % (Witnesses, ),
            r'$Nonwitnesses=%d$' % (Nonwitnesses, )))

        ax.hist(np_arr,200,range=(2,number-2))
        props = dict(boxstyle='round', facecolor='red', alpha=0.95)        
        ax.text(0.45, 0.15, textstr, transform=ax.transAxes, fontsize=14,verticalalignment='top', bbox=props)

        plt.title("Histogram of MR witnesses for N="+str(number))
        plt.ylabel("Number of witnesses per bin")
        plt.xlabel("Bins of size 200")
        plt.savefig(image,dpi=1200)
        plt.cla()
        


