---
layout: post
title: "A Bug's Life"
date: 2024-11-22 13:45:00 -0500
byline: How to triage from the ground up
categories: bug code
---

Let's set the scene: we can search for a person by name (first or last) and get a list of results, paginated (infinite scroll style) fifteen at a time. The full bug report: "sometimes people are missing from results".

## Step 1: Replicate

Log in as a super user to rule out any auth issues: if logged in at a lower level, people might be (correctly) missing from results because we don't have permission to see them. Start searching for people, try really popular names. A console error pops up; "list item has duplicate key" (this is a [React](https://react.dev/learn/rendering-lists#rules-of-keys) site, and the paginated infinite scroll just appends people to a list). Aha! Reading through the results, sure enough "Johnny Apples" shows up twice.

If there's a person loading twice, maybe they're taking the place of someone else? Could be a bit of a stretch, but it's surely odd, and enough to prove that pagination isn't working correctly. Plus, it's a lot easier to recognize duplicate people than some unknown someone that isn't there.

Log out, refresh the page, repeat all the steps above, and we get the same outcome. At this point, it's a good idea to update the Jira ticket with steps on how to replicate.

## Step 2: Investigate

Get the 'load more people' call from the browser's network tab and figure out what micro service handles it. Run that service locally and reproduce the call in Postman. Verify that at least one person is loaded at least twice; once on page two then again on page three.

Pull up the code and find the Postgres query that's fetching results, then run it manually in PgAdmin. We still get a duplicate person, but it's a different duplicate: Johnny Apple is a very common name in our system and this Johnny has a different id from the duplicate Johnny we found in Step 1 (any person's id can be found on the frontend by inspecting the element). 

## Step 3: Understand

So, running the same Postgres query manually returns different data than when it's run via a micro service. There must be something wrong with the query, or at least something unexpected is happening (assume that the code is never wrong, only our understanding of it). We start walking through the SQL line by line.

It turns out that `ORDER BY` is [not](https://stackoverflow.com/questions/67343360/is-order-by-stable-for-several-rows-with-the-same-key-values) [stable](https://stackoverflow.com/questions/15522746/is-sql-order-by-clause-guaranteed-to-be-stable-by-standards). So we're ordering by lastname then firstname, but it's not specific enough: people with the same first+last name could show up twice on different pages. And this is exactly the case for someone with a common name like Johnny Apples.

## Step 4: Solve

On the frontend, we only care about alphabetical order. It doesn't matter if Johnny Apples with id 502 shows up before Johnny Apples with id 345; it's up to the user to figure out which person they care about, and this order is currently (pseudo) random so the user has to verify every time.

We can add 'id' to the end of the `ORDER BY` list as a third discriminator after lastname and firstname.

Check out a branch, update the code, test it locally, commit and push, test on a development stack, update the Jira ticket, submit a PR, wait for review. This fix is small and easily understood so the PR passes first try. Merge our branch, update the ticket, submit the fix to QA, wait for review. It passes. Merge our branch one more time, update the ticket, then put it in a queue for next release.

## Step 5: Reflect

I like solving problems and thinking about code. The actual fix for this bug is very small, but it's all about the journey, not the destination. IMO, the last paragraph takes up too much time, and the more that that process could be automated, the better.

Having access to both source code and the database was crucial to fixing this bug, as well as the ability to run services locally. Knowledge of a platform's architecture is similarly indispensible; 'figure out what micro service handles it' could take weeks if poorly documented or misunderstood.

This fix may not have solved the original bug. We found a problem (duplicate people) that's slightly different from the original report (missing people), and wrote the replication steps ourselves. This bug might pop up at an entirely different time for an entirely different reason, perhaps when logged in as a non-super user. But we definitely fixed a bug, so that's a win. The platform is in a better state than when we started, and QA is happy.