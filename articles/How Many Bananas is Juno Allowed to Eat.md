# How Many Bananas is Juno Allowed to Eat?

There are only two hard problems in computer science: naming things, and cache invalidation.

We have several monkeys at the zoo, and all of them like to eat bananas. They would eat bananas all day if they could, so we keep track of how many they're allowed. We have a `Banana` service for that. To check how many bananas Juno is allowed to eat, we call `GET /bananas/juno`. To check how many bananas Marjorie is allowed to eat, we call `GET /bananas/marjorie`.

Some monkeys are allowed access to the secret forest. Some monkeys are allowed to eat cherries. Some monkeys are known to throw poo at visitors. We track it all at the zoo, and can conveniently fetch all monkey-related parameters through the `Wrapper` service, by calling `GET /wrapped/juno` for instance. This will return all the information we have on Juno. So instead of the zoo computer making several different calls to get banana limits, secret forest access, cherry limits, and poop complaints, it just makes one call to the bundler. Internally, the `Wrapper` service makes calls to a bunch of other services, including the `Banana` service.

One day, my colleague lowered the amount of bananas that Juno was allowed to eat (per day) from ten to nine, but when they refreshed the page it seemed like the change didn't go through; it still indicated that Juno was allowed to eat ten bananas. After a couple more attempts, finally the page updated to show nine bananas. We decided to investigate.

## Replication

It is known that both services cache their results for faster response times and to alleviate load on the database, and that cached values expire in five minutes if not invalidated sooner. Juno's banana limit is currently ten. Start by doing nothing for at least five minutes, then:

0. Call `GET /bananas/juno`, then start a stopwatch.

1. At minute one, keep waiting.

2. At two minutes, update Juno's banana limit from ten to nine, then call `GET /wrapped/juno`.

3. At three minutes, call `GET /bananas/juno` and note the result as *B1*.

4. At four minutes, keep waiting.

5. Just after five minutes, update Juno's banana limit from nine to eight. Call `GET /wrapped/juno` and note the result as *W1*.

Of course it would be easier to debug if we had direct access to the caching infrastructure, but unfortunately that wasn't the case.

The `Banana` and `Wrapper` caches are 'good' if their invalidation is working correctly, and 'bad' if not. Good caches will invalidate when Juno's banana limit changes. Bad caches will hold onto their value for five minutes after an initial call, no matter how Juno's banana limit changes. The table below summarizes all possible states of the two caches, and how the results would look.

<table>
<tbody>
<tr>
<td><code>Banana</code></td>
<td><code>Wrapper</code></td>
<td><em>B1</em></td>
<td><em>W1</em></td>
</tr>
<tr>
<td>good</td>
<td>good</td>
<td>9</td>
<td>8</td>
</tr>
<tr>
<td>good</td>
<td>bad</td>
<td>9</td>
<td>9</td>
</tr>
<tr>
<td>bad</td>
<td>good</td>
<td>10</td>
<td>8</td>
</tr>
<tr>
<td>bad</td>
<td>bad</td>
<td>10</td>
<td>10</td>
</tr>
</tbody>
</table>


During testing it was found that *B1* = 10 and *W1* = 10, so both caches are bad. Let's walk through how we got here:

0. We waited at least five minutes before this, so there's nothing cached yet. Calling `GET /bananas/juno` causes `Banana` to cache Juno's current banana limit of ten.

1. We wait.

2. Updating Juno's banana limit from ten to nine should invalidate both caches, but it doesn't. Calling `GET /wrapped/juno` will in turn call `GET /bananas/juno`, which responds with it's cached (and not invalidated) value of 'ten'. `Wrapper` will dutifully cache this value now. Both services have now cached a value, and both are incorrect. `Banana`'s cached value expires in three minutes, while `Wrapper`'s cached value will expire in five minutes (because it just got called).

3. When called, `Banana` returns it's cached value 'ten'. This is *B1*.

4. We wait.

5. `Banana`'s cached value has now expired; if called, `Banana` would go to the database and return a value of 'eight'. Instead we call `Wrapper`, and it returns it's cached (and not invalidated) value of ten.

Luckily, fixes were quickly implemented for both services.