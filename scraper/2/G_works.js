const { chromium } = require('playwright');
const fs = require('fs');

async function languageFiller(page, search_lang) {
    const searchInput = await page.$('input[type="text"][placeholder="Search..."]');
    await page.waitForTimeout(3000);
    await searchInput.fill(search_lang);
    await page.waitForTimeout(5000);
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


async function writeToFile(folder_dash, link, copied_solution, file_txt_underscore, folderPath) {
    // const folderPath = `${__dirname}/${folder_dash}`;
    // const filePath = `${folderPath}/${file_txt_underscore}.rs`;

    // const folderPath = `${__dirname}/${folderPath}/${folder_dash}`;
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
                            await writeToFile(folder_dash, link, copied_solution, file_txt_underscore, folderPath);

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
