Forget about the other absence function. I have why the file created in writeToFile function did not place inside "fs.mkdirSync(folderPath, { recursive: true });" ? Instead the writeToFile function just create the file as same path as  "fs.mkdirSync(folderPath, { recursive: true });"


Error :

nemesis@nemesis:~/Documents/Github/my_repo/leetcode-study-plan$ node A_bulk.js
Created folder: Array / String
Created file: /home/nemesis/Documents/Github/my_repo/leetcode-study-plan/Array / String/Merge Sorted Array
88. Merge Sorted Array
https://leetcode.com/problems/merge-sorted-array/solutions/?envType=study-plan-v2&envId=top-interview-150
https://leetcode.com/problems/merge-sorted-array/solutions
No sort button
Scraped 15 links
Rust button is visible or there is only one language
Error writing to file: Error: EISDIR: illegal operation on a directory, open '/home/nemesis/Documents/Github/my_repo/leetcode-study-plan/Array / String/Merge Sorted Array/merge_sorted_array1636648.rs'





code :


async function writeToFile(folder_dash, link, copied_solution, file_txt_underscore, filePath) {
    // const folderPath = `${__dirname}/${folder_dash}`;
    // const filePath = `${folderPath}/${file_txt_underscore}.rs`;

    // const folderPath = `${__dirname}/${folderPath}/${folder_dash}`;
    const filePath2 = `${filePath}/${file_txt_underscore}.rs`;

    try {
        // Create the folder if it doesn't exist
        fs.mkdirSync(filePath2, { recursive: true });

        // Write to the file
        fs.writeFileSync(filePath2, `// ${link}\n${copied_solution}`);

        console.log(`File ${file_txt_underscore} saved inside ${folder_dash} folder.`);
    } catch (err) {
        console.error(`Error writing to file: ${err}`);
    }
}




(async () => {
    const browser = await chromium.launch({ headless: false });
    const context = await browser.newContext();
    const page = await context.newPage();


    const search_lang = "rust";

    const unhide_Lang_Button = 'Rust';

    const target_language_class = '.language-rust, .language-python, .language-java, .language-cpp';


    const targetLink = 'https://leetcode.com/studyplan/top-interview-150/';
    await page.goto(targetLink);

    await page.waitForSelector('div.w-full.overflow-hidden.css-yw0m6t');

    // await detectFoldersAndCreateFiles(page);

    const folderElements = await page.$$('div.w-full.overflow-hidden.css-yw0m6t');

    for (const folderElement of folderElements) {

        const folderNameElement = await folderElement.$('div[style="font-size: 12px; font-weight: 500;"]');

        const folderName = await folderNameElement.textContent();
        const folderPath = `${__dirname}/${folderName}`;

        try {

            fs.mkdirSync(folderPath, { recursive: true });

            console.log(`Created folder: ${folderName}`);

            const fileElements = await folderElement.$$('div[style="font-weight: 500; font-size: 14px;"]');

            for (const element of fileElements) {
                const fileName = await element.textContent();

                // Update file path
                const filePath = `${folderPath}/${fileName}`;

                // fs.writeFileSync(filePath, '');
                fs.mkdirSync(filePath, { recursive: true });

                console.log(`Created file: ${filePath}`);


                // code

                await element.click();

                await new Promise((resolve) => setTimeout(resolve, 10 * 1000));

                const allPages = context.pages();

                const newPage = allPages[allPages.length - 1];

                // Extract main title
                const textContent = await newPage.$eval(
                    '.mr-2.text-label-1.dark\\:text-dark-label-1.text-lg.font-medium',
                    (element) => element.textContent
                );

                console.log(textContent);

                await newPage.waitForLoadState('networkidle');

                const solutionElement = await newPage.waitForSelector('span:has-text("Solutions")');

                await solutionElement.click();


                const folder_dash = `${textContent}`;

                // const folder_dash = `${folderPath}/${folder_name}`;

                console.log(newPage.url());

                const code1 = newPage.url();
                const urlParts = code1.split('/');

                // Remove the query parameters if they exist
                const lastPart = urlParts[urlParts.length - 1];
                const hasQueryParameters = lastPart.includes('?');

                const code2 = hasQueryParameters ? urlParts.slice(0, -1).join('/') : code1;

                console.log(code2);

                await newPage.goto(code2);

                try {
                    await languageFiller(newPage, search_lang);
                } catch (err) {
                    console.log('Page not found. Not premium user or there is no solution');

                    console.log('langugage filler error');
                    await new Promise(() => { });
                }

                try {
                    await sortButtonClicker(newPage);
                } catch (err) {
                    console.log('No sort button');
                }


                try {
                    await listOption(newPage);
                } catch (err) {
                    console.log('No list option');
                }


                try {
                    await recentButtonClicker(newPage)
                } catch (err) {
                    console.log('No recent button');
                }

                const links = await scrapEachSolutionLink(newPage);

                if (links.length === 0) {
                    console.log('No links found');
                } else {
                    for (const link of links) {
                        try {
                            const file_txt_underscore = await constructSolutionFile(newPage, link);

                            await clickLangButton(newPage, unhide_Lang_Button);

                            const copied_solution = await copySolutionToClipboard(newPage, target_language_class);
                            await writeToFile(folder_dash, link, copied_solution, file_txt_underscore, filePath);

                        } catch (error) {
                            console.log(`Error occurred while processing link: ${link}`);
                            console.error(error);
                        }
                    }
                }

                await newPage.close();


            }
        } catch (err) {
            console.error(`Error creating folder or file: ${err}`);
        }
    }

    await browser.close();
})();