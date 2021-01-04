### Rust implementation of a Java ConcurrentHashMap

This was a code-along based on a live stream by Jon Gjengset. <br>
I'm just trying to learn Rust. <br>
-Link to Jon's YouTube live stream [Click on me!!!](https://www.youtube.com/watch?v=yQFWmGaFBjk)

- The underlined data structure for ConcurrentHashMap is Hashtable.

  - Hash table maps keys to values.
  - To successfully store and retrieve objects from a hashtable, the objects used as keys must implement the hashCode method and the equals method.
  - ConcurrentHashMap class is thread-safe i.e. multiple threads can operate on a single object without any complications.
  - only lockes while adding or updating the map. But allows concurrent threads to **read the value without locking** at all.

  <img src="./hashMap.png" alt="drawing" width="700"/>
