




<!DOCTYPE html>
<html>
<head>
<title>Unified Diff Format</title>
<meta charset="utf-8">
<meta name="author" content="Guido van van Rossum" />
<link rel="stylesheet" type="text/css" href="/artima.css" />
<link rel="shortcut icon" href="/favicon.ico" />
</head>
<body><table width="100%" cellspacing="0">
<tr>
<td align="left" valign="bottom">
<a href="/index.jsp"><img src="/images/a7.gif" alt="The Artima Developer Community" border="0" width="550" height="43" /></a>
</td>
</table>
<table width="100%" bgcolor="#333333">
<tr>
<td align="center">
<div class="ml">
<a href="/articles" class="hl">Articles</a>&nbsp;|
<a href="/news/index.jsp" class="hl">News</a>&nbsp;|
<a href="/weblogs/index.jsp" class="hl">Weblogs</a>&nbsp;|
<a href="/shop/catalog" class="hl">Books</a>&nbsp;|
<a href="/forums/index.jsp" class="hl">Forums</a>
</div>
</td>
</tr>
</table>
<table width="100%" bgcolor="#AACCFF">
<tr>
<td align="center">
<div class="sc">
<a href="/weblogs/index.jsp">Artima Weblogs</a>&nbsp;| 

<a href="/weblogs/index.jsp?blogger=guido">Guido van van Rossum's Weblog</a>&nbsp;| 

<a href="/forums/flat.jsp?forum=106&thread=164293">Discuss</a>&nbsp;| 
<a href="mailto:?subject=Unified Diff Format&body= %0AArtima Weblogs %0AUnified Diff Format %0Aby Guido van van Rossum %0A%0Ahttps://www.artima.com/weblogs/viewpost.jsp?thread=164293">Email</a>&nbsp;| 
<a href="viewpostP.jsp?thread=164293">Print</a>&nbsp;| 
<a href="/weblogs/bloggers.jsp">Bloggers</a>&nbsp;| 
<a class= "sl" href="viewpost.jsp?thread=161207" title="Looking for Memories of Python Old-Timers">Previous</a></a>&nbsp;| 
<a class= "sl" href="viewpost.jsp?thread=165788" title="Upcoming Appearances">Next</a></a>
</div>
</td>
</tr>
</table>
<table width="100%" bgcolor="#EEEEEE">
<tr>
<td align="center">
<div class="sc">
<span style="color: #555555">Sponsored Link</span> <span style="color: #888888">&bull;</span> 
<script language='JavaScript' type='text/javascript' src='https://www.artima.com/zcr/adx.js'></script>
<script language='JavaScript' type='text/javascript'>
<!--
   if (!document.phpAds_used) document.phpAds_used = ',';
   phpAds_random = new String (Math.random()); phpAds_random = phpAds_random.substring(2,11);
   
   document.write ("<" + "script language='JavaScript' type='text/javascript' src='");
   document.write ("https://www.artima.com/zcr/adjs.php?n=" + phpAds_random);
   document.write ("&amp;what=zone:9&amp;target=_top");   document.write ("&amp;exclude=" + document.phpAds_used);
   if (document.referrer)
      document.write ("&amp;referer=" + escape(document.referrer));
   document.write ("'><" + "/script>");
//-->
</script><noscript><a href='https://www.artima.com/zcr/adclick.php?n=a799ecf6' target='_top'><img src='https://www.artima.com/zcr/adview.php?what=zone:9&amp;n=a0587811' border='0' alt=''></a></noscript>
</div>
</td>
</tr>
</table>
<BR>
<div class="vegies">
<div class="tc">
<span class="sts">All Things Pythonic</span><br />
<span class="ts">Unified Diff Format</span><br />
<span class="as">by Guido van van Rossum</span><br />
<span class="pd">June 14, 2006</span><br />
</div>




<blockquote>
<b>Summary</b><br>
I couldn't find a thorough spec for the format called &quot;unified diff&quot; so I decided to research it.  Here are my findings.

</blockquote>

<hr align="left" width="90%">



<table align="right">
<tr>
<td>
<div class="adnotice">
Advertisement
</div>
<center>
<script language='JavaScript' type='text/javascript' src='https://www.artima.com/zcr/adx.js'></script>
<script language='JavaScript' type='text/javascript'>
<!--
if (!document.phpAds_used) document.phpAds_used = ',';
phpAds_random = new String (Math.random()); phpAds_random = phpAds_random.substring(2,11);
document.write ("<" + "script language='JavaScript' type='text/javascript' src='");
document.write ("https://www.artima.com/zcr/adjs.php?n=" + phpAds_random);
document.write ("&amp;what=zone:2");
document.write ("&amp;exclude=" + document.phpAds_used);
if (document.referrer)
document.write ("&amp;referer=" + escape(document.referrer));
document.write ("'><" + "/script>");
//-->
</script><noscript><a href='https://www.artima.com/zcr/adclick.php?n=a74ab060' target='_blank'><img src='https://www.artima.com/zcr/adview.php?what=zone:2&amp;n=a74ab060' border='0' alt=''></a></noscript>

</center>
 </td>
</tr>
</table>

<p>
<p>I haven't found a satisfactory specification of the unified diff
format (the one on the GNU website is hopelessly incomplete).
Here's what I've discovered by experimenting with diff(1) on Red Hat
Linux; this identifies itself as 'diff (GNU diffutils) 2.8.1'.
Hopefully this is useful for someone who needs to generate unified
diffs or who needs to parse them.  (I had both needs recently. :-)</p>
<p>The header lines look like this:</p>
<pre class="literal-block">
indicator ' ' filename '\t' date ' ' time ' ' timezone
</pre>
<p>where:</p>
<blockquote>
<ul class="simple">
<li>indicator is '---' for the old file and '+++' for the new</li>
<li>date has the form YYYY-MM-DD</li>
<li>time has the form hh:mm:ss.nnnnnnnnn on a 24-hour clock</li>
<li>timezone is has the form ('+'|'-') hhmm where hhmm is hours and
minutes east (if the sign is +) or west (if the sign is -) of
GMT/UTC</li>
</ul>
</blockquote>
<p>Each chunk starts with a line that looks like this:</p>
<pre class="literal-block">
'&#64;&#64; -' range ' +' range ' &#64;&#64;'
</pre>
<p>where range is either one unsigned decimal number or two separated
by a comma.  The first number is the start line of the chunk in the
old or new file.  The second number is chunk size in that file; it
and the comma are omitted if the chunk size is 1.
(Email from a reader suggests that this omission is optional
and may be phased out.)  If the chunk size is
0, the first number is one lower than one would expect (it is the
line number after which the chunk should be inserted or deleted; in
all other cases it gives the first line number or the replaced range
of lines).</p>
<p>A chunk then continues with lines starting with ' ' (common line),
'-' (only in old file), or '+' (only in new file).  If the last line
of a file doesn't end in a newline character, it is displayed with a
newline characer, and the following line in the chunk has the
literal text (starting in the first column):</p>
<pre class="literal-block">
'\ No newline at end of file'
</pre>


<h1>Talk Back!</h1>

<p>
Have an opinion?


Readers have already posted

<a href="../forums/flat.jsp?forum=106&thread=164293">5

comments</a>
about this weblog entry. Why not

<a href="../forums/post.jsp?forum=106&thread=164293&reply=true">add yours</a>?


<h1>RSS Feed</h1>

<p>
If you'd like to be notified whenever Guido van van Rossum adds a new entry to <a href="index.jsp?blogger=guido">his weblog</a>, subscribe to his <a href="feeds/bloggers/guido.rss">RSS feed</a>.

<center>
<div class="sociallinks">
  <a href="http://digg.com/submit?phase=2&url=http%3A%2F%2Fwww.artima.com%2Fweblogs%2Fviewpost.jsp%3Fthread%3D164293&title=Unified+Diff+Format&bodytext=I+couldn%27t+find+a+thorough+spec+for+the+format+called+%26quot%3Bunified+diff%26quot%3B+so+I+decided+to+research+it.++Here+are+my+findings.&topic=programming">
    <img src="/images/digg.gif" alt="Digg"
         border="0" height="14" hspace="8" width="16" />Digg
  </a>
  |
  <a href="http://del.icio.us/post?url=http%3A%2F%2Fwww.artima.com%2Fweblogs%2Fviewpost.jsp%3Fthread%3D164293&title=Unified+Diff+Format">
    <img src="/images/delicious.gif" alt="del.icio.us" 
         border="0" height="16" hspace="8" width="16" vspace="1" />del.icio.us
  </a>
  |
  <a href="http://programming.reddit.com/submit?url=http%3A%2F%2Fwww.artima.com%2Fweblogs%2Fviewpost.jsp%3Fthread%3D164293&title=Unified+Diff+Format">
    <img src="/images/reddit.gif" alt="Reddit" 
         border="0" height="18" hspace="8" width="18" />Reddit
  </a>  
</div>
</center>

<h1>About the Blogger</h1>

<P>
<table><tr valign="bottom"><td><img src="../images/guido.jpg" align="right"></td><td>Guido van Rossum is the creator of Python, one of the major
programming languages on and off the web. The Python community refers to him as the BDFL (Benevolent Dictator For Life), a title straight
from a Monty Python skit. He moved from the Netherlands to the USA in
1995, where he met his wife. Until July 2003 they lived in the
northern Virginia suburbs of Washington, DC with their son Orlijn, who
was born in 2001. They then moved to Silicon Valley where Guido now works for Google
(spending 50% of his time on Python!).</td></tr></table><p>

<div class="sp">This weblog entry is Copyright &copy; 2006 Guido van van Rossum. All rights reserved.</div>
</div>

</div>
<hr width="100%" />
<table width="50%" align="center">
<tr>
<td>
<div class="horizontaltextadbox">
<div class="adheadline">Sponsored Links</div>
<div id="sponsoredlinks">
</div>
</div>
</td>
</tr>
</table>
<hr width="100%" />
<center>
<script type="text/javascript"><!--
google_ad_client = "pub-3911176865765226";
google_alternate_color = "ffffff";
google_ad_width = 728;
google_ad_height = 15;
google_ad_format = "728x15_0ads_al";
google_ad_channel = "";
google_color_border = "ffffff";
google_color_bg = "FFFFFF";
google_color_link = "003090";
google_color_text = "000000";
google_color_url = "666666";
//--></script>
<script type="text/javascript"
  src="https://pagead2.googlesyndication.com/pagead/show_ads.js">
</script>
<br />
<br />
<!-- SiteSearch Google -->
<form method="get" action="https://www.google.com/custom">
<table border="0" bgcolor="#ffffff">
<tr><td nowrap="nowrap" valign="top" align="left" height="32">
<a href="https://www.google.com/">
<img src="https://www.google.com/logos/Logo_25wht.gif"
border="0" alt="Google"></img></a>
</td>
<td nowrap="nowrap">
<input type="hidden" name="domains" value="Artima.com"></input>
<input type="text" name="q" size="31" maxlength="255" value=""></input>
<input type="submit" name="sa" value="Search"></input>
</td></tr>
<tr>
<td>&nbsp;</td>
<td nowrap="nowrap">
<font size="-1" color="#000000">
<input type="radio" name="sitesearch" value=""></input> Web
<input type="radio" name="sitesearch" value="Artima.com" checked="checked"></input>Artima.com
</font>&nbsp;&nbsp;
<input type="hidden" name="client" value="pub-3911176865765226"></input>
<input type="hidden" name="forid" value="1"></input>
<input type="hidden" name="ie" value="ISO-8859-1"></input>
<input type="hidden" name="oe" value="ISO-8859-1"></input>
<input type="hidden" name="cof" value="GALT:#008000;GL:1;DIV:#336699;VLC:663399;AH:center;BGC:FFFFFF;LBGC:FFFFFF;ALC:0000FF;LC:0000FF;T:000000;GFNT:0000FF;GIMP:0000FF;LH:50;LW:150;L:https://www.artima.com/images/artima150.gif;S:https://www.artima.com;FORID:1;"></input>
<input type="hidden" name="hl" value="en"></input>
</td></tr></table>
</form>
<!-- SiteSearch Google -->
</center>
<br />
<div class="sp">
<div style="text-align: center">
<a href="https://www.artima.com/copyright.html">Copyright</a> &copy; 1996-2019 Artima, Inc. All Rights Reserved.</a> - <a href="https://www.artima.com/privacy.html">Privacy Policy</a> - <a href="https://www.artima.com/termsofuse.html">Terms of Use</a>
</div>
</div>
<br />
<script language='JavaScript' type='text/javascript'>
<!--
function initBannerVarForZone(zone) {
        initBannerVarForZoneWithScript(zone, 'adjs_modified');
}

function initBannerVarForZoneWithScript(zone, phpScript) {

        if (!document.phpAds_used) document.phpAds_used = ',';
        phpAds_random = new String (Math.random());
        phpAds_random = phpAds_random.substring(2,11);

        var nextScriptSrc = 'https://www.artima.com/zcr/' + phpScript + '.php?n=' +
                phpAds_random  +
                '&amp;what=zone:' + zone + '&amp;target=_top&amp;block=1&amp;blockcampaign=1' +
                '&amp;exclude=' + document.phpAds_used;

        document.write("<script language='JavaScript' type='text/javascript' src='");
        document.write(nextScriptSrc);
        document.write("'><\/script>");

}

function replaceDiv(divID) {
        document.getElementById(divID).innerHTML = phpadsbanner;
}
-->
</script>
<script language='JavaScript' type='text/javascript'>
<!--
initBannerVarForZone(3);
-->
</script>

<script language='JavaScript' type="text/javascript">
<!--
replaceDiv('leftskyscraper');
-->
</script>
<script language='JavaScript' type='text/javascript'>
<!--
initBannerVarForZoneWithScript(4, 'textman');
-->
</script>

<script language='JavaScript' type="text/javascript">
<!--
replaceDiv('sponsoredlinks');
-->
</script>
</body>
</html>
