- Genomes are linear representations of network connectivity consist:
    - list of connection genes 
- Node genes consist:
    - list of inputs
    - hidden nodes
    - outputs 
- Connection consist:
    - in node
    - out node
    - weight connection
    - enable bit
    - innovation number
- Mutations can change:
    - weights
    - network structure
        - connect two node with random weight
        - add new node z between node u, v: 1. old connection disabled 2. w(u,z) = 1, w(z,v) = w(u,v)
- Historical Marking
    - each gene is assigned an innovation number from a global counter
- Speciation
    - delta = c1*E/N + c2*D/N +c3*W
    E = excess genes
    D = disjoint genes
    W = average weight difference of matching genes
    N = number of genes in larger genome (N can be 1 if both are <20)
    c1,c2,c3 are constant coefficients 
    - a list of species is kept 
      - existing species are represented by a random genome from previous generation
      - a genome g is placed in the first species that its delta is lower than some threshold 
      - if no species exists a new one is created with g being the representation genome 
- fitness function
    - f' = f / sum(sh(delta(i,j)))
      - sh 0 when delta > threshold
      - sh 1 otherwise
    - species offspring is dependent on sum of f' 
    - population is replaced by the offspring of the top members of each species
    - if the fitness of the entire population doesn't increase after 20 generations only the top 2 species reproduce

- initializing
    - the initial population is uniform all inputs connect directly to all outputs
