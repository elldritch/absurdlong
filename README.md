# absurdlong

`absurdlong` computes the longest game of [Absurdle](https://qntm.org/absurdle) you could possibly play while still guessing rationally.

A rational player:

- Never reuses black letters.
- Always reuses green letters in the same position.
- Always reuses yellow letters, but never in a previously yellowed position.

First, some obvious bounds:

- The answer is [at least 4](https://twitter.com/zwegner/status/1480110927752175618).
- The answer is at most 2315 (the number of possible target words).

Absurdle works by dividing all target words into one of 243 buckets depending on the guess response (each letter can have 1 of 3 responses, and there are 5 letters, therefore 3^5 guess responses), and picking the bucket with the most remaining target words.

<!-- Is it sufficient to always greedily pick the largest bucket? -->

Naively, we can search every possible game by trying every possible start word and playing out every possible tree.

Every tree's maximum length is the number of remaining possible target words in its bucket, so we can prune branches when a branch's maximum length is less than the length of another branch. We can traverse the game tree with depth-first search, keeping the longest branch seen so far.

We can go in alphabetical order, and checkpoint progress by serializing the current branch (since branch order is deterministic) and the current longest-known branch.
