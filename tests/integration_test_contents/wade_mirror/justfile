default:
    just --list --unsorted

watch:
    compile-typst-site --watch

build:
    compile-typst-site

export DEFAULT_BLOG_POST := '''
#import "../../templates/blog_post.typ": conf

#show: conf.with(
    page-title: "TODO",
    date: "1000-10-10",
)
'''
blog:
	printf '%s' "$DEFAULT_BLOG_POST" > src/blog/NEW_BLOG_POST.typ
	code src/blog/NEW_BLOG_POST.typ
