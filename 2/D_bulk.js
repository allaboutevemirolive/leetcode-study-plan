const { chromium } = require('playwright');
const fs = require('fs');

// async function detectFoldersAndCreateFiles(page) {

// }

(async () => {
    const browser = await chromium.launch({ headless: false });
    const context = await browser.newContext();
    const page = await context.newPage();

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

            for (const fileElement of fileElements) {
                const fileName = await fileElement.textContent();
                const filePath = `${folderPath}/${fileName}.txt`;

                // fs.writeFileSync(filePath, '');
                fs.mkdirSync(filePath, { recursive: true });

                console.log(`Created file: ${filePath}`);
            }
        } catch (err) {
            console.error(`Error creating folder or file: ${err}`);
        }
    }

    await browser.close();
})();
