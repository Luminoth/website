# 7/30/2015 Shane
1. Swapped Bootstrap nav for Material Design toolbar/sidenav
2. Now forcing 0.10.0 Material Design version (NuGet is behind)
3. Updated NuGet packages

# 7/25/2015 Shane
1. Moved from pure CSS to compiled SCSS

# 7/11/ 2015 Shane
1. Updating dependencies
2. Updating About page
3. Fixing carousel index tracking

# 6/30/2015 Shane
1. Fixing ReSharper warnings
2. Removing data- prefix across the board
3. Updated NuGet packages

# 5/22/2015 Shane
1. Adding resource error handling
2. Moving the downloads feed to the database

# 5/21/2015 Shane
1. RSS feed is now dynamically generated from the database
2. Database tables are now dropped on deploy

# 5/20/2015 Shane
1. News is now pulled from a new database
2. Adding an Angular filter for converting JSON date strings to JavaScript Date objects
3. Index and Test pages are now MVC Razor Views
4. Adding ASP.NET Web Optimization NuGet package and (sort of) making use of it
5. Adding a 500 error handler
6. Making use of Bundles
7. Separating out the layout into an MVC layout

# 5/19/2015 Shane
1. Download page is now built from a JSON resource
2. Feed is now Atom instead of RSS
3. Added a copy of the Deathwatch character generator

# 5/18/2015 Shane
1. Using Angular Material Design grids in place of Bootstrap grids
2. Adding a 404 route
3. General cleanup
4. Adding a debug test route
5. Updated Angular NuGet package
6. Fixed the WoW addons page not showing the last WoW version
7. Updated CSS class names
8. Removed Bootstrap content-fluid class from body content

# 5/11/2015 Shane
1. Reorganized TypeScript files a little bit
2. Updated the news and RSS feed

# 5/10/2015 Shane
1. Removed non-TypeScript Angular/Bootstrap NuGet packages. They aren't really useful over using a CDN right now
2. Doing another cleanup pass

# 5/9/2015 Shane
1. Converted all Angular code to TypeScript (1.4)
2. Simplified the way the Oct2005Cabin vacation page is generated
3. Moving the default page into the DefaultController

# 5/8/2015 Shane
1. Updating NuGet Packages
2. Adding the following NuGet packages: AngularJS.Core/Resource/Route/Sanitize/UI Bootstrap, angularjs.TypeScript.DefinitelyTyped
  * Changed a few of the CDN-served scripts to use the local scripts
3. Moved JSON files to App_Data so they're no longer directly accessible
  * Added a default controller for serving these files
5. Switched from calling MapRoute() to using MVC Route Attributes
6. Added a Product identifier to the Launcher update
  * Also updated the "test" URL and added a static directory to go along with it

# 5/7/2015 Shane
1. Making better use of Bootstrap rows/columns in order to cleanup padding issues

# 5/6/2015 Shane
1. Changed Launcher Date types to string
2. Renamed Launcher News Timestamp to Date

# 5/2/2015 Shane
1. Making the page footer sticky

# 5/1/2015 Shane
1. Kennel page is now fetched through Angular routing
2. Unicode page is now fetched through Angular routing
3. Removed unused WoW screenshots views
4. Raw XP page is now fetched through Angular routing
5. Notepad and socket tutorials are now fetched through Angular routing
6. Oct 2005 vacation page is now fetched through Angular routing
7. Changed WoW macros over to use Angular resources
8. WoW pages are now fetched through Angular routing
9. Adding a 404 view
10. Removed the useless dinner calendar
11. Site is now hosted on Azure
12. Updated structure to better reflect life as an ASP.NET project
  * Removed all references to Django

# 4/30/2015 Shane
1. kennel page now uses a ui.bootstrap carousel
2. WoW screenshots now use ui.bootstrap carousel
3. Separated the home page from the news page
4. The main set of pages now use Angular routing instead of Django views
5. General CSS/layout cleanup
6. Removed tables from the main set of pages and started making use of Bootstrap grids
7. Removed the pointless scheduler app

# 4/29/2015 Shane
1. Making the site design mobile-friendly using Angular and ui.bootstrap
2. Swapped to using ui.bootstrap with templates
3. oct2005cabin vacation now uses a ui.bootstrap carousel

# 4/28/2015 Shane
1. Changed image rollover to be an Angular directive
2. Moved Google Analytics script to bottom of the layout where it belongs
3. Fixed a bug in the addons page where ng-href wasn't being used
4. Changed UTF-8 endcoding to ASCII on all of the template files
5. Fixed some bugs in the oct2005cabin template

# 4/27/2015 Shane
1. Removing the unused feedreader view
2. Adding Underscore.js
3. kennel page is now built with angular instead of as a view
4. Reorganized the oct2005cabin vacation images (again)
5. Removed jQuery in favor of using Angular for everything
6. Replaced Bootstrap.js with Angular's UI-Bootstrap module
7. Adding a layout debugging view

# 4/26/2015 Shane
1. Moved vacation images for consistency
2. Renamed CSS/RSS files to be more "standard"
3. Adding Bootstrap and Respond to the set of libraries used

# 4/24/2015 Shane
1. Switching to use ngResource
2. Added static index pages
3. Removing PHP files in favor of static files and django templates
4. Removed PHP support from Apache config
5. Dinner calendar is now built with angular
6. Updated the October 2005 cabin trip page to be a template
7. Removed old vacation url from Apache
8. WoW addon page is now built with angular and json

# 4/23/2015 Shane
1. Moving to use jQuery
2. A little bit of cleanup
3. HTML5 fixes
4. AngularJS-ifying the homepage

# 4/14/2015 Shane
1. Updated about page

# 4/13/2015 Shane
1. Renamed app application to home
2. Fixed file permissions
3. Fixed the deploy script
4. Updated httpd.conf and fixed vacation URL
5. Fixed the WSGI file
6. Fixed production settings
7. Switched over to using the static keyword
8. Changing how static files are collected
9. Fixed some old media references
10. Added missing scheduler templates
11. Added apache restart to the deploy script
12. Fixed the lanparty

# 4/12/2015 Shane
1. Rebuilt energonsoftware repository as new EnergonSoftware repository
2. Setup VisualStudio project
