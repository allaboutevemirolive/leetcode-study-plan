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

    }

    await browser.close();
})();
