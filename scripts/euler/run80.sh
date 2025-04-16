#!/bin/bash
sbatch <<EOT
#!/bin/bash

#SBATCH --output=out80.txt
#SBATCH --error=out80.txt
#SBATCH --mem-per-cpu=32GB
#SBATCH --time=9-00:00:00
#SBATCH --gpus=a100_80gb:1

module load stack/2024-06 openssl/3.1.3-zhfub4o cuda/12.1.1 gperftools/2.13
./zkbench.sh $@

exit 0
EOT
