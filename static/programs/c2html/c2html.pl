#! /usr/bin/perl -w

#####
# todo:
#    optimizations
#    full keyword list
#    special HTML characters
#####

use strict;
my $version     = "0.6.0";
my $script_name = "c2html.pl";  # this must match the name of the script
my $copy_years  = "&copy;2001-2003";
my $home        = "http://energon.home.mindspring.com/";
my $company     = "Energon Software";

#
# replace these colors with your prefered
#

my $bg_color   = '"#ffffff"';
my $text_color = '"#000000"';

my $multi = "false";
sub parse_line
{
    #
    # replace these colors with your prefered
    #
    my $key_color     = '"#0000cc"';
    my $string_color  = '"#cc0000"';
    my $comment_color = '"#007700"';

    my @preprocessor = ( "#if defined", "#if !defined", "#elif defined",
                         "#include", "#define", "#undef", "#line", "#error", "#pragma",
                         "#if", "#ifdef", "#ifndef", "#elif", "#else", "#endif" );

    my @keywords = ( "and", "and_eq", "asm", "auto", "bitand", "bitor",
                     "bool", "break", "case", "catch", "char", "class",
                     "compl", "const", "const_cast", "continue", "default", "delete",
                     "do", "double", "dynamic_cast", "else", "enum", "explicit",
                     "export", "extern", "false", "float", "for", "friend",
                     "goto", "if", "inline", "int", "long", "mutable",
                     "namespace", "new", "not", "not_eq", "operator", "or",
                     "or_eq", "private", "protected", "public", "register", "reinterpret_cast",
                     "return", "short", "signed", "sizeof", "static", "static_cast",
                     "struct", "switch", "template", "this", "throw", "true",
                     "try", "typedef", "typeid", "typename", "union", "unsigned",
                     "using", "virtual", "void", "volatile", "wchar_t", "while",
                     "xor", "xor_eq", "_asm", "NULL" );

    shift && chomp;
    s/</&lt;/g && s/>/&gt;/g;  # <> characters
    if((/\/\*/) || ($multi eq "true")) {    # working w/ multiline comments
        $multi = "true";
        if(/\/\*/) {
            $_ = "<font color=$comment_color>\n".$_;
        }
        if(/\*\//) {
            $_.="</font>";
            $multi = "false";
        }
    } else {                                # not working w/ multiline comments
        s/(\".+?\")/<font color=$string_color>$1<\/font>/g;      # strings
        s/(\'.+?\')/<font color=$string_color>$1<\/font>/g;      # characters
        s/(&lt;.+?&gt;)/<font color=$string_color>$1<\/font>/g;  # standard headers

        foreach my $preproc (@preprocessor) {
            s/$preproc/<font color=$key_color>$preproc<\/font>/g;
        }
        foreach my $keyword (@keywords) {
            s/\b$keyword\b/<font color=$key_color>$keyword<\/font>/g;
        }

        # pull all font tags out of comments
        if(/\/\//) {
            s/<font color=\".*\">//g;
            s/<\/font>//g;
        }
        s/(\/\/.*)/<font color=$comment_color>$1<\/font>/g;
    }

    return;
}

sub optimize_line
{
    chomp;
    return $_."\n";
}

sub main
{
    my $html = "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01 Transitional//EN\">\n<html>\n<head>\n    <meta name=\"generator\" content=\"$script_name v$version\">\n    <meta http-equiv=\"Content-type\" content=\"text/html; charset=iso-8859-1\">\n\n    <title>";
    my $copyright = "\n<hr>\n\nSource Converted by $script_name $copy_years <a href=\"$home\">$company</a>";

    my $file = shift;
    unless(open INFILE, $file) {
        print "Could not open $file\n";
        return;
    }

    $html.="        $file by $script_name</title>\n</head>\n\n<body bgcolor=$bg_color text=$text_color>\n\n<pre>\n";

    print "Converting $file...\n";
    my $line;
    while(<INFILE>) {
        &parse_line;
        $line = &optimize_line;
        $html.=$line;
    }
    close INFILE;

    # fix file extension
    $file =~ s/\./_/g;
    my $htmlfile = $file.".html";

    $file.=".html";
    print "Writing $htmlfile...\n";
    unless(open OUTFILE, ">$htmlfile") {
        print "Could not create $htmlfile\n";
        return;
    }
    print OUTFILE $html.$copyright."\n</pre>\n\n</body>\n</html>";
    close OUTFILE;

    print "\n";
}

die "Usage: $script_name [file1] [file2] ...\n" unless @ARGV;
foreach my $file(@ARGV) {
    &main($file) unless($file eq $script_name);
}
exit 0;
