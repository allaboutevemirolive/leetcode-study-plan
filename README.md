
Generate nodejs code with playwright that do the following task :


https://leetcode.com/studyplan/top-interview-150/


In the link above there is many  "style="font-weight: 500; font-size: 14px;""  like in the html below...

Detetct all of them and scrap it, then place them in a txt file


Scrap this html :

<div class="max-w-[85%] " style="font-weight: 500; font-size: 14px;"><div class="truncate">Merge Sorted Array</div></div>


2. Get the string in it


___


Generate nodejs code with playwright that do the following task :



Target link : https://leetcode.com/studyplan/top-interview-150/



There are many of this html in the page. Detect all of them.

<div class="w-full overflow-hidden css-yw0m6t">



Inside each of html above, there is exist the html below.

Detect this html below and create folder baser on the text inside it

<div style="font-size: 12px; font-weight: 500;">Array / String</div>



Under the same html "<div class="w-full overflow-hidden css-yw0m6t">"... there are many of this html below.

Detect all of them and create file for each of it, based on the text inside it and place them inside the folder that has been created before.

```javascript
const elements = await page.$$('div[style="font-weight: 500; font-size: 14px;"]');
```

