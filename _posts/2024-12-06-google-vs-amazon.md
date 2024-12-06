---
layout: post
title: "Stevey's Google Platforms Rant"
date: 2024-12-06 09:44:00 -0500
link: https://gist.github.com/chitchcock/1281611
byline: Amazon does platforms better than anyone else
tags: item
---
Steve Yagge: 

> His Big Mandate went something along these lines:
> 1. All teams will henceforth expose their data and functionality through service interfaces.
> 2. Teams must communicate with each other through these interfaces.
> 3. There will be no other form of interprocess communication allowed: no direct linking, no direct reads of another team's data store, no shared-memory model, no back-doors whatsoever. The only communication allowed is via service interface calls over the network.
> 4. It doesn't matter what technology they use. HTTP, Corba, Pubsub, custom protocols -- doesn't matter. Bezos doesn't care.
> 5. All service interfaces, without exception, must be designed from the ground up to be externalizable. That is to say, the team must plan and design to be able to expose the interface to developers in the outside world. No exceptions.
> 6. Anyone who doesn't do this will be fired.
> 7. Thank you; have a nice day!  
> 
> Ha, ha! You 150-odd ex-Amazon folks here will of course realize immediately that #7 was a little joke I threw in, because Bezos most definitely does not give a shit about your day.

...

> Well, the first big thing Bezos realized is that the infrastructure they'd built for selling and shipping books and sundry could be transformed an excellent repurposable computing platform. So now they have the Amazon Elastic Compute Cloud, and the Amazon Elastic MapReduce, and the Amazon Relational Database Service, and a whole passel' o' other services browsable at aws.amazon.com. These services host the backends for some pretty successful companies, reddit being my personal favorite of the bunch.

> The other big realization he had was that he can't always build the right thing. I think Larry Tesler might have struck some kind of chord in Bezos when he said his mom couldn't use the goddamn website. It's not even super clear whose mom he was talking about, and doesn't really matter, because nobody's mom can use the goddamn website. In fact I myself find the website disturbingly daunting, and I worked there for over half a decade. I've just learned to kinda defocus my eyes and concentrate on the million or so pixels near the center of the page above the fold.

> I'm not really sure how Bezos came to this realization -- the insight that he can't build one product and have it be right for everyone. But it doesn't matter, because he gets it. There's actually a formal name for this phenomenon. It's called Accessibility, and it's the most important thing in the computing world.

...

> That one last thing that Google doesn't do well is Platforms. We don't understand platforms. We don't "get" platforms. Some of you do, but you are the minority. This has become painfully clear to me over the past six years. I was kind of hoping that competitive pressure from Microsoft and Amazon and more recently Facebook would make us wake up collectively and start doing universal services. Not in some sort of ad-hoc, half-assed way, but in more or less the same way Amazon did it: all at once, for real, no cheating, and treating it as our top priority from now on.

...

> A product is useless without a platform, or more precisely and accurately, a platform-less product will always be replaced by an equivalent platform-ized product.