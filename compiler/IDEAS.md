I want to generate links between posts. I don't want to use AI. A random button? A random link? Two random links? Should the random links change every time posts are generated? Two known links and one random? Generate similar posts based on vector embeddings? Or even simpler, just a count of similar words? How to prevent stagnation, where posts just point to eachother? We want a small world graph. So I think an easy way to do that is to add randomness. Or maybe link to the most similar, the median similar, and the least similar? What other metrics could be used? Word count. Some sort of sentiment analysis. Date / time of upload. No, it's got to come down to the words and their meanings. What about similar links? Do we care about the content of posts? Posts that involve quotes in them (so naively look for '>', or hook into the parser?)? And maybe generate one / several aggregate pages, based on grouping or similarity or word clouds like a wordpress blog. Or rank posts based on the similarity of the frequency of words used in the posts?

I want to re-do the directory structure, to allow for different post types. So:
- a 'site' folder that holds all my markdown stuff for the site. Assume everything else in this list is there
- a single md at the root will be converted into index.html. If more than one file: error
- can have any sub directories you want, but only 1 level deep. So 'posts', 'thoughts', 'books', 'lists'
- everything in deirectory X is compiled into /pages/X/1 and pages/X/2 and pages/X/3 and so on

