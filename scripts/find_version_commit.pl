#!/usr/bin/perl

# @author Bodo (Hugo) Barwich
# @version 2023-02-07
# @package TextSanitizer
# @subpackage scripts/find_version_commit.pl

# This Module parses the Git History to find the Merge Commit for the Version
#

use strict;
use warnings;

use Git;
use Data::Dump qw(dump);

#----------------------------------------------------------------------------
#Auxiliary Functions

sub parse_commit_line {

}

#----------------------------------------------------------------------------
#Executing Section

my $ierr = 0;

my $repo           = Git->repository( Directory => '.git' );
my @commit_history = ();
my %commits        = ();
my $commit         = undef;
my $commit_idx     = 0;

my @log_lines = $repo->command( 'log', '-50' );

for my $line (@log_lines) {
    if ( $line =~ qr/^commit (.*)$/i ) {
        print "cmt: '$1'\n";

        $commit = {
            'hash'       => $1,
            'hash_short' => substr( $1, 0, 7 ),
            'raw'        => $line . "\n",
            'index'      => $commit_idx
        };

        $commits{ $commit->{'hash_short'} } = $commit;

        push @commit_history, ($commit);

        $commit_idx++;
    }
    else {
        $commit->{'raw'} .= $line . "\n";

        if ( index( $line, ':' ) != -1 ) {
            if ( $line =~ qr/^([^:]+): (.*)/ ) {
                $commit->{ lc $1 } = $2;
            }
        }
    }
}

for $commit (@commit_history) {
    if ( defined $commit->{'author'} ) {
        if ( $commit->{'author'} =~ qr/^([^<]+) <([^>]+)>/ ) {
            $commit->{'author'} = { 'name' => $1, 'email' => $2 };
        }
    }

    if ( defined $commit->{'date'} ) {
        $commit->{'date'} =~ s/^\s+//;
    }
}

print "cmts dmp:\n", dump( \@commit_history ), "\n";
