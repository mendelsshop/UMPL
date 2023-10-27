#include <stdio.h>
#include <stdlib.h>
typedef struct llNode
{
    int data;
    struct llNode *next;
} llNode;

typedef struct treeNode
{
    int data;
    struct treeNode *left;
    struct treeNode *right;
} treeNode;

typedef struct callInfo
{
    int argc;
    void *jmp;
} callInfo;

treeNode *proccess_var_args(int left, llNode *var_args);
void print_tree_args(treeNode *root);
int printff(callInfo info, llNode *args);
void *jmp;

int main(int argc, char const *argv[])
{

    /* code */
    callInfo info;
    info.jmp = NULL;
    // init jmp
    printff(info, NULL);
    // info.jmp = jmp;
    // printff(info, NULL);
    // return 0;
    llNode *root = malloc(sizeof(treeNode));
    root->data = 1;
    root->next = malloc(sizeof(treeNode));
    root->next->data = 2;
    root->next->next = malloc(sizeof(treeNode));
    root->next->next->data = 3;
    info.argc = 3;
    // info.jmp = NULL;
    printff(info, root);
}

int printff(callInfo info, llNode *args)
{
    if (jmp == NULL)
    {
        jmp = &&early_exit;
        return 0;
    }
    if (info.jmp != NULL)
    {
        goto *(info.jmp);
    }

    if (info.argc < 1)
    {
        printf("arrity error\n");
        exit(1);
    }
    int first = args->data;
    args = args->next;
    printf("%d, ", first);
    treeNode *i = proccess_var_args(info.argc-1 , args);
    print_tree_args(i);
    return 0;
early_exit:
    printf("you skipped printing");
    return 1;
}

treeNode *proccess_var_args(int left, llNode *var_args)
{
    if (left <= 0)
    {
        return NULL;
    }
    left = left / 2;
    treeNode *root = malloc(sizeof(treeNode));
    root->left = proccess_var_args(left, var_args);
    root->data = var_args->data;
    if (var_args->next == NULL)
    {
        return root;
    }
    *var_args = *var_args->next;
    root->right = proccess_var_args(left, var_args);
    return root;
}

void print_tree_args(treeNode *root)
{
    if (root == NULL)
    {
        return;
    }
    print_tree_args(root->left);
    printf("%d, ", root->data);
    print_tree_args(root->right);
}