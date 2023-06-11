const { chromium } = require('playwright');
const fs = require('fs');

(async () => {
    const browser = await chromium.launch({ headless: false });

    // Create a new browser context
    const context = await browser.newContext();

    let page = await context.newPage();

    await page.goto('https://leetcode.com/studyplan/top-interview-150/');

    await page.waitForTimeout(10 * 1000);

    const elements = await page.$$('div[style="font-weight: 500; font-size: 14px;"]');

    for (const element of elements) {
        await element.click();

        await new Promise((resolve) => setTimeout(resolve, 10 * 1000));

        // Get the list of all pages in the browser context
        const allPages = context.pages();

        // Get the last opened page (new popup)
        const newPage = allPages[allPages.length - 1];

        // Wait for the new page to load
        await newPage.waitForLoadState('networkidle');


        await new Promise(() => { });

        // let solutionButton = await newPage.$$('span[Solutions]');

        // await solutionButton.click();


        // Extract the text content from the specified HTML element
        const textContent = await newPage.$eval(
            '.mr-2.text-label-1.dark\\:text-dark-label-1.text-lg.font-medium',
            (element) => element.textContent
        );

        console.log(textContent);

        // Close the new page
        await newPage.close();
    }

    await browser.close();
})();
