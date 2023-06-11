const { chromium } = require('playwright');
const fs = require('fs');
const { URL } = require('url');


async function languageFiller(page, search_lang) {
    const searchInput = await page.$('input[type="text"][placeholder="Search..."]');
    await page.waitForTimeout(3000);
    await searchInput.fill(search_lang);
    await page.waitForTimeout(5000);
}


async function accessQuestionLink(page) {
    // const targetUrl = match + "solutions/?orderBy=newest_to_oldest";


    const url = new URL(page);
    const pathSegments = url.pathname.split('/');

    // Find the index of the "description" path segment
    const descriptionIndex = pathSegments.findIndex(segment => segment === 'description');

    if (descriptionIndex !== -1) {
        // Replace the "description" segment with "solutions"
        pathSegments[descriptionIndex] = 'solutions';

        // Update the URL pathname with the modified path segments
        url.pathname = pathSegments.join('/');

        const targetUrl = url.origin + url.pathname;
        console.log(targetUrl);
    } else {
        console.log("Invalid URL format. Unable to convert.");
    }


    await page.goto(targetUrl);
    await page.waitForTimeout(3000);
}


// Need to refactor this function
async function folderPageNotFound(folder_page_not_found) {
    console.log('No links found');

    const folderPath = `${__dirname}/${folder_page_not_found}`;
    const filePath = `${folderPath}/emptyFile.txt`;

    try {
        // Create the folder if it doesn't exist
        fs.mkdirSync(folderPath, { recursive: true });

        // Write to the file
        fs.writeFileSync(filePath, `// nothing`);

        console.log(`File 'emptyFile' saved inside ${folder_page_not_found} folder.`);
    } catch (err) {
        console.error(`Error writing to file: ${err}`);
    }
}


async function sortButtonClicker(page) {
    const button_sort = await page.$('#headlessui-menu-button-\\:Rmaa9j9l5t6\\:');
    await button_sort.click();
}

async function listOption(page) {
    const button_option = await page.$('button[id="headlessui-menu-button-:Rb5ai9j9d5t6:"]');
    await button_option.click();
}


async function recentButtonClicker(page) {
    const button_recent = await page.$('div.truncate:has-text("Most Recent")');
    await button_recent.click();
}


async function scrapEachSolutionLink(page) {

    const links = await page.$$eval('a[href*="/problems/"]', links =>
        links.filter(link => {
            const row = link.closest('div[class="hover:bg-fill-4 dark:hover:bg-dark-fill-4 relative flex w-full cursor-pointer gap-4 px-4 py-4"]');
            return row && row.querySelector('a[href*="/problems/"]') === link;
        })
            .map(link => link.href)
    );

    console.log(`Scraped ${links.length} links`);

    return links;
}

// Need to refactor this function
async function constructSolutionFile(page, link) {

    await page.goto(link);
    await page.waitForTimeout(3000);

    const pathSegments = link.split('/').filter(str => str !== "");
    // We need to change "-" to "_" to follow the naming convention
    const problemName = pathSegments[pathSegments.indexOf("problems") + 1];
    const updatedProblemName = problemName.replace(/-/g, '_');

    const solutionId = pathSegments[pathSegments.indexOf("solutions") + 1];
    const file_txt_underscore = `${updatedProblemName}${solutionId}`;

    return file_txt_underscore;
}


async function clickLangButton(page, unhide_Lang_Button) {
    const targetLanguageButton = await page.$(`div.relative.cursor-pointer.px-3.py-3.text-label-4.dark\\:text-dark-label-4.hover\\:text-label-1.dark\\:hover\\:text-dark-label-1.GMIHh:has-text("${unhide_Lang_Button}")`);

    if (targetLanguageButton) {
        await targetLanguageButton.click();
        console.log('Clicked Rust hidden button / there is more than one language');
    } else {
        console.log('Rust button is visible or there is only one language');
    }
}


async function copySolutionToClipboard(page, target_language_class) {
    const dataElement = await page.waitForSelector(target_language_class);
    await page.waitForTimeout(3000);
    const copied_solution = await dataElement.innerText();
    await page.waitForTimeout(3000);
    return copied_solution;
}


// Need to refactor this function
async function writeToFile(folder_dash, link, copied_solution, file_txt_underscore) {
    const folderPath = `${__dirname}/${folder_dash}`;
    const filePath = `${folderPath}/${file_txt_underscore}.rs`;

    try {
        // Create the folder if it doesn't exist
        fs.mkdirSync(folderPath, { recursive: true });

        // Write to the file
        fs.writeFileSync(filePath, `// ${link}\n${copied_solution}`);

        console.log(`File ${file_txt_underscore} saved inside ${folder_dash} folder.`);
    } catch (err) {
        console.error(`Error writing to file: ${err}`);
    }
}





(async () => {
    const browser = await chromium.launch({ headless: false });

    const context = await browser.newContext();

    let page = await context.newPage();

    const search_lang = "rust";

    const unhide_Lang_Button = 'Rust';

    const target_language_class = '.language-rust, .language-python, .language-java, .language-cpp';

    await page.goto('https://leetcode.com/studyplan/top-interview-150/');

    await page.waitForTimeout(10 * 1000);

    const elements = await page.$$('div[style="font-weight: 500; font-size: 14px;"]');

    for (const element of elements) {
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

        // Code you need to implement
        const solutionElement = await newPage.waitForSelector('span:has-text("Solutions")');
        // const solutionText = await solutionElement.textContent();

        await solutionElement.click();

        // Implement language filler

        const folder_dash = `${textContent}`;


        // const url = new URL(newPage.url());
        // url.pathname = url.pathname.replace('/description/', '/solutions/');
        // const code2 = url.toString();

        console.log(newPage.url());

        const code1 = newPage.url();
        const urlParts = code1.split('/');

        // Remove the query parameters if they exist
        const lastPart = urlParts[urlParts.length - 1];
        const hasQueryParameters = lastPart.includes('?');

        const code2 = hasQueryParameters ? urlParts.slice(0, -1).join('/') : code1;

        console.log(code2);

        await newPage.goto(code2);

        // try {
        //     const url = new URL(newPage.url());
        //     const pathSegments = url.pathname.split('/');

        //     // Find the index of the "description" path segment
        //     const descriptionIndex = pathSegments.findIndex(segment => segment === 'description');

        //     if (descriptionIndex !== -1) {
        //         // Replace the "description" segment with "solutions"
        //         pathSegments[descriptionIndex] = 'solutions';

        //         // Update the URL pathname with the modified path segments
        //         url.pathname = pathSegments.join('/');

        //         const targetUrl = url.origin + url.pathname;

        //         await newPage.goto(targetUrl);


        //         console.log(targetUrl);
        //     } else {
        //         console.log("Invalid URL format. Unable to convert.");
        //     }
        // } catch (err) {
        //     console.log('Page not found. Not premium user or there is no solution');
        // }

        try {
            // await accessQuestionLink(newPage);
            await languageFiller(newPage, search_lang);
        } catch (err) {
            console.log('Page not found. Not premium user or there is no solution');
            // await folderPageNotFound(folder_page_not_found);

            console.log('langugage filler error');
            await new Promise(() => { });
            // continue;
        }

        // Future deprecate sortButtonClicker
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

        // await new Promise(() => { });

        const links = await scrapEachSolutionLink(newPage);

        // links for the solution
        if (links.length === 0) {
            // If page not found, create an empty folder with " _EMPTY_ " prefix
            // await folderPageNotFound(folder_page_not_found);
            console.log('No links found');
        } else {
            // For each solution/link, we store it in a txt file
            for (const link of links) {
                try {
                    const file_txt_underscore = await constructSolutionFile(newPage, link);

                    await clickLangButton(newPage, unhide_Lang_Button);

                    const copied_solution = await copySolutionToClipboard(newPage, target_language_class);
                    await writeToFile(folder_dash, link, copied_solution, file_txt_underscore);

                } catch (error) {
                    console.log(`Error occurred while processing link: ${link}`);
                    console.error(error);
                }
            }
        }


        // console.log(solutionText);

        // await new Promise(() => { });

        await newPage.close();
    }

    await browser.close();
})();
