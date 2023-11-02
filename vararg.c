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

typedef struct helper
{
    treeNode *tree;
    llNode *next;
} helper;

typedef struct callInfo
{
    int argc;
    void *jmp;
} callInfo;
helper proccess_var_args(int left, llNode *var_args);
void print_tree_args(treeNode *root);
int printff(callInfo info, llNode *args);
void *jmp;

int main(int argc, char const *argv[])
{

    /* code */
    callInfo info;
    printff(info, NULL);
    llNode *root = malloc(sizeof(treeNode));
    root->data = 2;
    root->next = malloc(sizeof(treeNode));
    root->next->data = 3;
    root->next->next = malloc(sizeof(treeNode));
    root->next->next->data = 4;
    root->next->next->next = malloc(sizeof(treeNode));
    root->next->next->next->data = 5;
    root->next->next->next->next = malloc(sizeof(treeNode));
    root->next->next->next->next->data = 6;

    info.argc = 5;
    printff(info, root);
}

int printff(callInfo info, llNode *args)
{

    // if (info.jmp != NULL)
    // {
    //     goto *(info.jmp);
    // }

    if (info.argc < 0)
    {
        printf("arrity error\n");
        exit(1);
    }
    // int first = args->data;
    // args = args->next;
    // printf("%d, ", first);
    treeNode *i = proccess_var_args(info.argc, args).tree;
    print_tree_args(i);
    return 0;
}

// how we proccess variables arguments
helper proccess_var_args(int left, llNode *var_args)
{
    if (left <= 0)
    {
        helper ret = {
            .tree = NULL,
            .next = var_args};
        return ret;
    }
    int mid = left / 2;
    treeNode *root = malloc(sizeof(treeNode));
    helper lefts = proccess_var_args(mid, var_args);
    root->left = lefts.tree;
    root->data = lefts.next->data;
    helper rights = proccess_var_args(left - mid - 1, lefts.next->next);
    root->right = rights.tree;
    helper ret = {
        .tree = root,
        .next = rights.next};
    return ret;
}

void print_tree_args(treeNode *root)
{
    if (root == NULL)
    {
        return;
    }
    print_tree_args(root->left);
    printf("<-%d->", root->data);
    print_tree_args(root->right);
    free(root);
}