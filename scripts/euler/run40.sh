#!/bin/bash
sbatch <<EOT
#!/bin/bash

#SBATCH --output=out.txt
#SBATCH --error=out.txt
#SBATCH --mem-per-cpu=32GB
#SBATCH --time=5-00:00:00
#SBATCH --gpus=1
#SBATCH --gres=gpumem:40g

module load stack/2024-06 openssl/3.1.3-zhfub4o cuda/12.1.1 gperftools/2.13
./zkbench.sh $@

exit 0
EOT
