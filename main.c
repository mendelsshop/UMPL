
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

struct object;
struct cons;
typedef struct hempty
{
} hempty;
typedef struct cons
{
   struct object *car;
   struct object *cdr;
   struct object *cgr;
} cons;

typedef struct object
{
   int type;
   bool boolean;
   double number;
   char *string;
   struct cons tree;
   hempty hempty;
} object;

typedef struct node
{
   object data;
   struct node *next;
} node;
void new_hempty_inner(object *n)
{
   n->type = 4;
   hempty h;
   n->hempty = h;
}
object new_hempty()
{
   object n;
   new_hempty_inner(&n);
   return n;
}

void new_int_inner(object *n, double d)
{
   n->type = 1;
   n->number = d;
}

object new_int(double d)
{
   object n;
   new_int_inner(&n, d);
   return n;
}

void new_tree_inner(object *root, double d)
{
   cons root_tree;
   object n;
   root_tree.car = malloc(sizeof(object));
   root_tree.cdr = malloc(sizeof(object));
   root_tree.cgr = malloc(sizeof(object));
   // *(root_tree.cdr) = new_int(d);
   // *(root_tree.cgr) = new_hempty();
   // *(root_tree.car) = new_hempty();
   new_hempty_inner(root_tree.car);
   new_int_inner(root_tree.cdr, d);
   new_hempty_inner(root_tree.cgr);
   root->type = 3;
   root->tree = root_tree;
}

void new_tree_init_inner(object *root, object car, object cdr, object cgr)
{
   cons root_tree;
   object n;
   root_tree.car = malloc(sizeof(object));
   root_tree.cdr = malloc(sizeof(object));
   root_tree.cgr = malloc(sizeof(object));
   *(root_tree.car) = car,
   *(root_tree.cdr) = cdr,
   *(root_tree.cgr) = cgr;
   root->type = 3;
   root->tree = root_tree;
}

object new_tree_init(object car, object cdr, object cgr)
{
   object root;
   new_tree_init_inner(&root, car, cdr, cgr);
   return root;
}
object new_tree(double d)
{
   object n;
   new_tree_inner(&n, d);
   return n;
}

void print_object(object n)
{
   switch (n.type)
   {
   case 0:
      printf("%d", n.boolean);
      break;
   case 1:
      printf("%.2f", n.number);
      break;
   case 2:
      printf("%s", n.string);
      break;
   case 3:
      printf("(");
      print_object(*(n.tree.car));
      printf(" ");
      print_object(*(n.tree.cdr));
      printf(" ");
      print_object(*(n.tree.cgr));
      printf(")");
      break;
   case 4:
      printf("hempty");
      break;
   }
}

bool is_hempty(object o)
{
   return o.type == 4;
}
typedef struct helper
{
   object data;
   struct helper *next;
} helper;

void print_helper(helper *h)
{
   helper *h1 = malloc(sizeof(helper));
   h1 = h;
print_entry:
{
   printf("%p->", h1);
   if (h1 == NULL)
   {
      goto done;
   }
   else
      goto print_actual;
}
print_actual:
   print_object(h1->data);
   h1 = h1->next;
done:
   printf("()\n");
   return;
}

void iter(object o)
{

   object *n = malloc(sizeof(struct object));
   *n = o;
   helper **helper_iter = malloc(sizeof(helper **));
   *helper_iter = NULL;
loop_entry:
   if (is_hempty(*n))
   {
      goto loop_swap;
   }
   else
   {
      goto loop_process;
   }

loop_process:
   if (n->type == 3)
   {
      goto loop_procss_1;
   }
   else
   {
      goto cons_error;
   }

loop_procss_1:
   if (is_hempty(*n->tree.car))
   {
      goto loop;
   }
   else
   {
      goto loop_save;
   }

loop:
   print_object(*n->tree.cdr);
   printf("\n");
   *n = *n->tree.cgr;
   goto loop_entry;

loop_save:
{
   helper *new = malloc(sizeof(helper));
   new->data = new_tree_init(new_hempty(), *n->tree.cdr, *n->tree.cgr);
   new->next = *helper_iter;
   *helper_iter = new;
   *n = *n->tree.car;
   goto loop_entry;
}
loop_swap:
{
   bool is_helper_null = (*helper_iter) == NULL;
   if (is_helper_null)
   {
      goto loop_done;
   }
   else
   {
      goto loop_swap_1;
   }
}
loop_swap_1:
   *n = (*helper_iter)->data;
   *helper_iter = (*helper_iter)->next;
   goto loop_entry;

cons_error:
   printf("Error:\nnon cons [");
   print_object(*n);
   printf("]\n");
   exit(1);

loop_done:
   return;
}

int main()
{
   object root = new_tree(5.);
   new_tree_inner(root.tree.car, 4.);
   new_tree_inner(root.tree.car->tree.car, 2.);
   new_tree_inner(root.tree.car->tree.car->tree.car, 1.);
   new_tree_inner(root.tree.car->tree.car->tree.cgr, 3.);
   *root.tree.car->tree.cgr = new_int(1);
   new_tree_inner(root.tree.cgr, 9.);
   new_tree_inner(root.tree.cgr->tree.car, 7.);
   new_tree_inner(root.tree.cgr->tree.car->tree.car, 6.);
   new_tree_inner(root.tree.cgr->tree.car->tree.cgr, 8.);
   new_tree_inner(root.tree.cgr->tree.cgr, 10.);
   print_object(root);
   printf("\n");
   iter(root);
}
