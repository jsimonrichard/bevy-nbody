Store a BVH for each chunk
Store all BVHs as a single resource, run update BVH system before movement update code.
Create new BVH when object moves into new chunk (manage with HashMap)

Systems:
* Chunk categorization: move bodies to the right BVH
* For each BVH: update based on new objects, objects that have left, and object movement
* For each object: use the BVHs that are near to calculate forces
